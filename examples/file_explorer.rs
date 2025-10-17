//! File explorer example using the Tree widget

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use tuxtui::prelude::*;
use tuxtui::widgets::block::{Block, BorderType};
use tuxtui::widgets::tree::{Tree, TreeNode, TreeState};

struct App {
    tree_state: TreeState,
}

impl App {
    fn new() -> Self {
        let mut state = TreeState::new();
        state.select(Some("root".to_string()));
        Self {
            tree_state: state,
        }
    }

    fn toggle_expand(&mut self, nodes: &mut Vec<TreeNode>) {
        if let Some(selected_id) = self.tree_state.selected() {
            Self::toggle_node_by_id(nodes, selected_id);
        }
    }

    fn toggle_node_by_id(nodes: &mut [TreeNode], id: &str) -> bool {
        for node in nodes {
            if node.id == id {
                node.expanded = !node.expanded;
                return true;
            }
            if Self::toggle_node_by_id(&mut node.children, id) {
                return true;
            }
        }
        false
    }

    fn select_next(&mut self, nodes: &[TreeNode]) {
        let flat = flatten_for_selection(nodes);
        if let Some(selected_id) = self.tree_state.selected() {
            if let Some(pos) = flat.iter().position(|id| id == selected_id) {
                if pos + 1 < flat.len() {
                    self.tree_state.select(Some(flat[pos + 1].clone()));
                }
            }
        } else if !flat.is_empty() {
            self.tree_state.select(Some(flat[0].clone()));
        }
    }

    fn select_previous(&mut self, nodes: &[TreeNode]) {
        let flat = flatten_for_selection(nodes);
        if let Some(selected_id) = self.tree_state.selected() {
            if let Some(pos) = flat.iter().position(|id| id == selected_id) {
                if pos > 0 {
                    self.tree_state.select(Some(flat[pos - 1].clone()));
                }
            }
        }
    }
}

fn flatten_for_selection(nodes: &[TreeNode]) -> Vec<String> {
    let mut result = Vec::new();
    for node in nodes {
        result.push(node.id.clone());
        if node.expanded {
            result.extend(flatten_for_selection(&node.children));
        }
    }
    result
}

fn create_demo_tree() -> Vec<TreeNode<'static>> {
    vec![
        TreeNode::new("📁 src", "root")
            .expanded(true)
            .child(
                TreeNode::new("📁 widgets", "widgets")
                    .expanded(true)
                    .child(TreeNode::new("📄 block.rs", "block"))
                    .child(TreeNode::new("📄 paragraph.rs", "paragraph"))
                    .child(TreeNode::new("📄 list.rs", "list"))
                    .child(TreeNode::new("📄 tree.rs", "tree"))
            )
            .child(
                TreeNode::new("📁 layout", "layout")
                    .child(TreeNode::new("📄 constraint.rs", "constraint"))
                    .child(TreeNode::new("📄 flex.rs", "flex"))
            )
            .child(TreeNode::new("📄 lib.rs", "lib"))
            .child(TreeNode::new("📄 buffer.rs", "buffer")),
        TreeNode::new("📁 examples", "examples")
            .child(TreeNode::new("📄 hello_world.rs", "hello"))
            .child(TreeNode::new("📄 file_explorer.rs", "explorer")),
        TreeNode::new("📄 Cargo.toml", "cargo"),
        TreeNode::new("📄 README.md", "readme"),
    ]
}

fn main() -> std::io::Result<()> {
    let mut terminal = tuxtui::init()?;
    let mut app = App::new();
    let mut nodes = create_demo_tree();
    
    let result = run(&mut terminal, &mut app, &mut nodes);
    tuxtui::restore()?;
    result
}

fn run(terminal: &mut tuxtui::DefaultTerminal, app: &mut App, nodes: &mut Vec<TreeNode>) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            let block = Block::default()
                .title("File Explorer (↑/↓: navigate, Enter: expand/collapse, q: quit)")
                .borders(BorderType::All);

            let inner = block.inner(area);
            frame.render_widget(block, area);

            let tree = Tree::new(nodes.clone())
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

            tree.render_stateful(inner, frame.buffer_mut(), &mut app.tree_state);
        })?;

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') => break Ok(()),
                KeyCode::Down => app.select_next(nodes),
                KeyCode::Up => app.select_previous(nodes),
                KeyCode::Enter => app.toggle_expand(nodes),
                _ => {}
            }
        }
    }
}
