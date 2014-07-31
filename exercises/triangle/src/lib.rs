struct Point {
    ...
}

enum TriangleResult {
    ...
}

struct Triangle {
    ...
}

impl Triangle {
    fn new(a: Point, b: Point, c: Point) -> TriangleResult {
        ...   
    }
}

#[cfg(test)]
mod test {
    fn test_good() {
        let a = Point{x: 1, y: 1};
        let b = Point{x: 2, y: 1};
        let c = Point{x: 2, y: 2};
        match Triangle::new(a, b, c) {
            Ok(triangle) => {},
            Error(e) => fail!()
        }
    }
    
    fn test_bad() {
        let a = Point{x: 1, y: 1};
        let b = Point{x: 2, y: 1};
        let c = Point{x: 1, y: 1};
        match Triangle::new(a, b, c) {
            Ok(triangle) => fail!(),
            Error(e) => {}
        }
    }
    
}
