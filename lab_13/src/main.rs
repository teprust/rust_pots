/// Узел двоичного дерева.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// Возможно пустое поддерево.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

/// Контейнер сохраняющий множество значений, с помощью двоичного дерева.
///
/// Если одно значение добавляется несколько раз, сохраняется только одно.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self { root: Subtree::new() }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

/// Методы new, insert, len, и has для `Subtree`.

impl<T:Ord> Subtree<T>{

    fn new() -> Self {
        // Создаём пустое поддерево
        Subtree(None)
    }

    /// Вставляем новое значение в поддерево
    /// Если значение уже существует, оно не добавляется
    fn insert(&mut self, value: T) {
        match self.0 {
            Some(ref mut node) => {
                if value < node.value {
                    // Вставляем в левое поддерево, если значение меньше текущего
                    node.left.insert(value);
                } else if value > node.value {
                    // Вставляем в правое поддерево, если значение больше текущего
                    node.right.insert(value);
                }
            }
            // Если значение равно текущему, оно не добавляется (множество не допускает дубликатов)
            None => {
                // Если текущий узел пуст, создаём новый узел с этим значением
                self.0 = Some(Box::new(Node {
                    value,
                    left: Subtree::new(),
                    right: Subtree::new(),
                }));
            }
        }
    }

    /// Возвращаем количество элементов в поддереве
    fn len(&self) -> usize {
        match self.0 {

            Some(ref node)
                // Подсчитываем элементы рекурсивно: текущий узел + элементы в левом и правом поддеревьях
                => 1 + node.left.len() + node.right.len(),
            None
                // Если узел пуст, его размер 0
                => 0,
        }
    }

    /// Проверяем присутствие указанного значения в поддереве
    fn has(&self, value: &T) -> bool {
        match self.0 {
            Some(ref node) => {
                // Нашли искомый элемент?
                if value == &node.value {
                    true
                } else if value < &node.value {
                    // Ищем в левом поддереве, если значение меньше текущего
                    node.left.has(value)
                } else {
                    // Ищем в правом поддереве, если значение больше текущего
                    node.right.has(value)
                }
            }
            // Если узел пуст, значит элемента нет
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> =
                (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}