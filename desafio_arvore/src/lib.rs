// Definição da estrutura do nó da árvore
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Definição da estrutura da BST
struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    // Criar uma nova árvore vazia
    fn new() -> Self {
        BST { root: None }
    }

    // Verificar se a árvore está vazia
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // Inserir um valor na árvore
    fn insert(&mut self, value: i32) {
        self.root = Self::insert_recursive(self.root.take(), value);
    }

    // Função auxiliar para inserir um valor na árvore
    fn insert_recursive(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        match node {
            None => Some(Box::new(Node {
                value,
                left: None,
                right: None,
            })),
            Some(mut node) => {
                if value < node.value {
                    node.left = Self::insert_recursive(node.left.take(), value);
                } else if value > node.value {
                    node.right = Self::insert_recursive(node.right.take(), value);
                }
                Some(node)
            }
        }
    }

    // Buscar um valor na árvore
    fn search(&self, value: i32) -> bool {
        Self::search_recursive(&self.root, value)
    }

    // Função auxiliar para buscar um valor na árvore
    fn search_recursive(node: &Option<Box<Node>>, value: i32) -> bool {
        match node {
            None => false,
            Some(node) => {
                if value == node.value {
                    true
                } else if value < node.value {
                    Self::search_recursive(&node.left, value)
                } else {
                    Self::search_recursive(&node.right, value)
                }
            }
        }
    }
}

#[cfg(test)]
mod bst_tests {
    use super::BST;

    #[test]
    fn test_bst_new_and_empty() {
        let bst = BST::new();
        assert!(bst.is_empty());
    }

    #[test]
    fn test_bst_insert_and_search() {
        let mut bst = BST::new();

        bst.insert(10);
        bst.insert(5);
        bst.insert(15);

        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));

        assert!(!bst.search(20));

        assert!(!bst.is_empty());
    }
}

