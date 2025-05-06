use crate::style::StyledNode;

struct Dimensions {
    // position of the content area relative to the document origin:
    content: Rect,

    // Surrounding edges:
    padding: EdgeSize,
    border: EdgeSize,
    margin: EdgeSize,
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct EdgeSize {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

struct LayoutBox<'a> {
    dimensions: Dimensions,
    box_type: BoxType<'a>,
    children: Vec<LayoutBox<'a>>,
}

enum BoxType<'a> {
    BlockNode(&'a StyledNode<'a>),
    InlineNode(&'a StyledNode<'a>),
    AnonymousBlock,
}

impl LayoutBox {
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            dimensions: Default::default(),
            box_type,
            children: Vec::new(),
        }
    }

    fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BoxType::BlockNode(_) => self.layout_block(containing_block),
            BoxType::InlineNode(_) => {}
            BoxType::AnonymousBlock => {}
        }
    }

    fn get_inline_container(&mut self) -> &mut LayoutBox {
        match self.box_type {
            InlineNode(_) | AnonymousBlock => self,
            BlockNode(_) => {
                match self.children.last() {
                    Some(&LayoutBox {
                        box_type: AnonymousBlock,
                        ..
                    }) => {}
                    _ => self.children.push(LayoutBox::new(AnonymousBlock)),
                }
                self.children.last_mut().unwrap()
            }
        }
    }
}

fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
    // create the root box
    let mut root = LayoutBox::new(match style_node.display() {
        Block => BlockNode(style_node),
        Inline => InlineNode(style_node),
        DisplayNone => panic!("Root node has display: none."),
    });

    // create the descendant boxes.
    for child in &style_node.children {
        match child.display() {
            Block => root.children.push(build_layout_tree(child)),
            Inline => root
                .get_inline_container()
                .children
                .push(build_layout_tree(child)),
            DisplayNone => {} // Skip nodes with `display: none;`
        }
    }
    root
}
