#![feature(test)]
extern crate test;
#[cfg(test)]
mod tests {

    use test::Bencher;
    use std::collections::LinkedList;

    #[test]
    fn it_works() {
    }

    #[bench]
    fn bench_linked_list_traversal(b: &mut Bencher) {
        let mut list = LinkedList::new();
        for e in (0..1000) {
            list.push_back(e);
        }
        let mut sum = 0;
        b.iter(move || {
            for i in list.iter() { sum = sum + i; };
            sum
        });
    }

    #[bench]
    fn bench_array_traveral(b: &mut Bencher) {
        let mut ary = Vec::new();
        for e in (0..1000) {
            ary.push(e);
        }
        let mut sum = 0;
        b.iter(move || {
            for i in ary.iter() { sum = sum + i; };
            sum
        });
    }

}
