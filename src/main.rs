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


struct DataIter {
	data: Data,
	pos: i32
}

impl Iterator for DataIter {
    type Item=i32;
    fn next(&mut self) -> Option<i32>{
        match self.pos {
            0 => { self.pos+=1; Some(self.data.a)}
            1 => { self.pos+=1; Some(self.data.b)}
            2 => { self.pos+=1; Some(self.data.c)}
            _ => None
        }
    }
}


impl IntoIterator for Data{
    type Item=i32;
    type IntoIter = DataIter;
    fn into_iter(self) -> DataIter{
        DataIter{data: self, pos: 0}
    }
}

fn main() {
	let d = Data::new(1, 2, 3);
	for a in d{
		println!("{}", a);
    }
}
