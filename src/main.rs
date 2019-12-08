struct Data{
	a: i32,
	b: i32,
	c: i32
}

impl Data {
    fn new(a:i32, b:i32, c:i32) -> Self{
        Data{a,b,c}
    }
}


struct DataIter<'a> {
	data: &'a Data,
	pos: i32
}

impl Iterator for DataIter<'_> {
    type Item=i32;
    fn next(&mut self) -> Option<Self::Item>{
        match self.pos {
            0 => { self.pos+=1; Some(self.data.a)}
            1 => { self.pos+=1; Some(self.data.b)}
            2 => { self.pos+=1; Some(self.data.c)}
            _ => None
        }
    }
}

impl<'a> IntoIterator for &'a Data{
    type Item=i32;
    type IntoIter = DataIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        DataIter{data:&self, pos: 0}
    }
}

fn main() {
	let d = Data::new(1, 2, 3);
	for a in &d{
		println!("{}", a);
    }
    for a in &d{
		println!("{}", a);
    }
}
