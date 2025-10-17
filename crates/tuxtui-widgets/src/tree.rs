//! Tree widget for hierarchical data display with expand/collapse.

use alloc::string::String;
use alloc::vec::Vec;
use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::style::{Style, Stylize};
use tuxtui_core::terminal::Widget;
use tuxtui_core::text::Line;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A node in the tree structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode<'a> {
    /// The content to display
    pub content: Line<'a>,
    /// Child nodes
    pub children: Vec<TreeNode<'a>>,
    /// Whether this node is expanded
    pub expanded: bool,
    /// Node identifier (for selection)
    pub id: String,
}

impl<'a> TreeNode<'a> {
    /// Create a new tree node.
    #[must_use]
    pub fn new<T: Into<Line<'a>>, S: Into<String>>(content: T, id: S) -> Self {
        Self {
            content: content.into(),
            children: Vec::new(),
            expanded: false,
            id: id.into(),
        }
    }

    /// Add a child node.
    #[must_use]
    pub fn child(mut self, child: TreeNode<'a>) -> Self {
        self.children.push(child);
        self
    }

    /// Set expanded state.
    #[must_use]
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Check if this node has children.
    #[must_use]
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// State for a tree widget.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TreeState {
    /// Currently selected node ID
    selected: Option<String>,
    /// Scroll offset
    offset: usize,
}

impl TreeState {
    /// Create a new tree state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            selected: None,
            offset: 0,
        }
    }

    /// Get the selected node ID.
    #[must_use]
    pub fn selected(&self) -> Option<&str> {
        self.selected.as_deref()
    }

    /// Select a node by ID.
    pub fn select(&mut self, id: Option<String>) {
        self.selected = id;
    }

    /// Get the scroll offset.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Set the scroll offset.
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
}

/// Symbols used for tree rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeSymbols {
    /// Vertical line
    pub vertical: &'static str,
    /// Horizontal line  
    pub horizontal: &'static str,
    /// T-junction (├)
    pub branch: &'static str,
    /// L-junction (└)
    pub corner: &'static str,
    /// Expanded indicator
    pub expanded: &'static str,
    /// Collapsed indicator
    pub collapsed: &'static str,
}

impl Default for TreeSymbols {
    fn default() -> Self {
        Self {
            vertical: "│",
            horizontal: "─",
            branch: "├",
            corner: "└",
            expanded: "▼",
            collapsed: "▶",
        }
    }
}

/// A tree widget for hierarchical data.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::tree::{Tree, TreeNode};
///
/// let root = TreeNode::new("Root", "root")
///     .expanded(true)
///     .child(TreeNode::new("Child 1", "child1"))
///     .child(TreeNode::new("Child 2", "child2"));
///
/// let tree = Tree::new(vec![root])
///     .highlight_style(Style::default().bg(Color::Blue));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree<'a> {
    nodes: Vec<TreeNode<'a>>,
    style: Style,
    highlight_style: Style,
    symbols: TreeSymbols,
}

impl<'a> Tree<'a> {
    /// Create a new tree.
    #[must_use]
    pub fn new(nodes: Vec<TreeNode<'a>>) -> Self {
        Self {
            nodes,
            style: Style::new(),
            highlight_style: Style::new(),
            symbols: TreeSymbols::default(),
        }
    }

    /// Set the overall style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the highlight style for the selected node.
    #[must_use]
    pub const fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    /// Set the tree symbols.
    #[must_use]
    pub const fn symbols(mut self, symbols: TreeSymbols) -> Self {
        self.symbols = symbols;
        self
    }

    /// Flatten tree nodes for rendering.
    fn flatten_nodes(
        &self,
        nodes: &[TreeNode<'a>],
        prefix: &str,
        _is_last: bool,
    ) -> Vec<(String, Line<'a>, usize)> {
        let mut result = Vec::new();

        for (idx, node) in nodes.iter().enumerate() {
            let is_node_last = idx == nodes.len() - 1;

            // Build the prefix for this node
            let node_prefix = if prefix.is_empty() {
                String::new()
            } else {
                let connector = if is_node_last {
                    self.symbols.corner
                } else {
                    self.symbols.branch
                };
                alloc::format!("{}{}{} ", prefix, connector, self.symbols.horizontal)
            };

            // Add expansion indicator if has children
            let expansion = if node.has_children() {
                if node.expanded {
                    self.symbols.expanded
                } else {
                    self.symbols.collapsed
                }
            } else {
                " "
            };

            let display_line = alloc::format!("{}{} {}", node_prefix, expansion, node.content);
            result.push((node.id.clone(), Line::from(display_line), prefix.len()));

            // Add children if expanded
            if node.expanded {
                let child_prefix = if prefix.is_empty() {
                    String::new()
                } else {
                    let continuation = if is_node_last {
                        "  "
                    } else {
                        self.symbols.vertical
                    };
                    alloc::format!("{}{} ", prefix, continuation)
                };
                result.extend(self.flatten_nodes(&node.children, &child_prefix, is_node_last));
            }
        }

        result
    }

    /// Render the tree with state.
    pub fn render_stateful(self, area: Rect, buf: &mut Buffer, state: &mut TreeState) {
        if area.area() == 0 {
            return;
        }

        let flat_nodes = self.flatten_nodes(&self.nodes, "", false);

        // Adjust offset to ensure selected item is visible
        if let Some(selected_id) = &state.selected {
            if let Some(pos) = flat_nodes.iter().position(|(id, _, _)| id == selected_id) {
                if pos < state.offset {
                    state.offset = pos;
                } else if pos >= state.offset + area.height as usize {
                    state.offset = pos.saturating_sub(area.height as usize - 1);
                }
            }
        }

        // Render visible nodes
        let visible = flat_nodes
            .iter()
            .skip(state.offset)
            .take(area.height as usize);

        for (i, (id, line, _depth)) in visible.enumerate() {
            let y = area.top() + i as u16;
            let is_selected = state.selected.as_ref() == Some(id);

            let item_style = if is_selected {
                self.style.patch(self.highlight_style)
            } else {
                self.style
            };

            let mut x = area.left();
            for span in &line.spans {
                let span_style = item_style.patch(span.style);
                x = buf.set_string(x, y, &span.content, span_style);
                if x >= area.right() {
                    break;
                }
            }
        }
    }
}

impl Widget for Tree<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = TreeState::default();
        self.render_stateful(area, buf, &mut state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_tree_node_creation() {
        let node = TreeNode::new("Test", "test");
        assert_eq!(node.id, "test");
        assert!(!node.has_children());
    }

    #[test]
    fn test_tree_node_with_children() {
        let node = TreeNode::new("Root", "root").child(TreeNode::new("Child", "child"));
        assert!(node.has_children());
        assert_eq!(node.children.len(), 1);
    }

    #[test]
    fn test_tree_state() {
        let mut state = TreeState::new();
        assert_eq!(state.selected(), None);

        state.select(Some("test".to_string()));
        assert_eq!(state.selected(), Some("test"));
    }
}
