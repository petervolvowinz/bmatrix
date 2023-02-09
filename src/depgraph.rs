//Implements a dependency graph using boolean matrix
pub mod graph{
    use crate::booleanmatrix::Matrix;

    pub struct Graph{
        data: Matrix,
        labels: Vec<String>,
        colors: Vec<u8>;
        visited: Vec<bool>,
    }

    pub fn NewGraph(n:usize) -> Graph{
        let mut graph = Graph{
            data: Matrix::NewMatrix(n,n),
            labels: Vec::new(),
            visited: Vec::new(),
        };
        graph.labels.resize(n,String::with_capacity(10));
        graph.visited.resize(n,false);

        return graph;
    }

    impl Graph{
        pub fn AddEdge(&mut self,from:usize,to:usize){
            self.data.Set(from,to,true);
        }

        //remove edge from -> to
        pub fn RemoveEdge(&mut self,from:usize,to:usize){
            self.data.Set(from,to,false);
        }

        pub fn AddLabel(&mut self,from:usize,label:String){
            self.labels[from] = label;
        }

        pub fn Print(&self){
            self.data.Print();
        }

        pub fn PrintLabels(&self){
            for i in 0..self.labels.len(){
                println!("{}: {}",i,self.labels[i]);
            }
        }

        pub fn PrintVisited(&self){
            for i in 0..self.visited.len(){
                println!("{}: {}",i,self.visited[i]);
            }
        }

        pub fn DFS(&mut self,from:usize){
            self.visited[from] = true;
            for i in 0..self.data.n{
                if self.data.Get(from,i) && !self.visited[i]{
                    self.DFS(i);
                }
            }
        }

        pub fn DFSAll(&mut self) {
            for i in 0..self.data.n {
                if !self.visited[i] {
                    self.DFS(i);
                }
            }
        }

        //color graph, greedy DFS solution
        pub fn color(&mut self){
            let mut color = 0;
            for i in 0..self.data.n{
                if !self.visited[i]{
                    self.colors[i] = color;
                    self.DFS(i);
                    color += 1;
                }
            }
        }

        //unset all visited flags
        pub fn reset(&mut self){
            for i in 0..self.visited.len(){
                self.visited[i] = false;
            }
        }

        //return png image of graph
        pub fn toImage(&self) -> image::DynamicImage{
            let mut imgbuf = image::ImageBuffer::new(self.data.n as u32, self.data.n as u32);
            for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                if self.data.Get(x as usize,y as usize){
                    *pixel = image::Rgb([0, 0, 0]);
                }
            }
            return imgbuf;
        }
    }
}