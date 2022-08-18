use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
struct GOL {
    dimensions: (u32, u32),
    tempArray: Vec<Vec<u8>>,
    gameState: Vec<Vec<u8>>,
}

#[wasm_bindgen]
impl GOL {
    pub fn new(x: u32, y: u32) -> GOL {
        // Initializing 2 arrays with zeros.
        let mut tempGameState: Vec<Vec<u8>> = Vec::new();
        let mut tempTempArray: Vec<Vec<u8>> = Vec::new();

        let mut arrWithZeros = vec![];
        for _ in 0..x {
            arrWithZeros.push(0 as u8);
        }


        for _ in 0..y {
            tempGameState.push(arrWithZeros.clone());
            tempTempArray.push(arrWithZeros.clone());
        }

        // tempGameState = vec![
        //     vec![0,0,0,0,0,0,0,0],
        //     vec![0,0,0,0,0,0,0,0],
        //     vec![0,0,0,0,0,0,0,0],
        //     vec![0,0,0,0,1,0,0,0],
        //     vec![0,0,0,1,0,0,0,0],
        //     vec![0,0,1,0,0,0,0,0],
        //     vec![0,0,0,0,0,0,0,0],
        //     vec![0,0,0,0,0,0,0,0],
        // ];

        GOL {
            dimensions: (x, y),
            tempArray: tempTempArray,
            gameState: tempGameState,
        }
    }

    pub fn setSize(x: u32, y: u32) -> bool {
        if x > 2000 || y > 2000 {
            false
        } else {
            true
        }
    }

    pub fn setPixel(&mut self, x: usize, y: usize) -> bool {
        self.gameState[y][x] = 1;
        true
    }

    pub fn getState(&self) -> String {
        format!("{:?}", self)
    }

    pub fn step(&mut self) {
        for i in 1..self.dimensions.1 - 1 {
            for j in 1..self.dimensions.0 - 1 {
                let n_count = getNeighbourCount(&self.gameState, j as usize, i as usize); 
                if self.gameState[j as usize][i as usize] == 1 {
                    if n_count < 2 {
                        self.tempArray[i as usize][j as usize] = 0;
                    }

                    else if n_count == 2 || n_count == 3 {
                        self.tempArray[i as usize][j as usize] = 1;
                    }

                    else if n_count > 3 {
                        self.tempArray[i as usize][j as usize] = 0;
                    }
                }
                if n_count == 3 {
                    self.tempArray[i as usize][j as usize] = 1;
                }
            }
        }

        self.gameState = self.tempArray.clone();
    }
}

fn getNeighbourCount(arr: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let mut count = 0;

    if arr[y + 1][x - 1] > 0 {
        count += 1;
    }
    if arr[y + 1][x] > 0 {
        count += 1;
    }
    if arr[y + 1][x + 1] > 0 {
        count += 1;
    }


    if arr[y][x - 1] > 0 {
        count += 1;
    }
    if arr[y][x + 1] > 0 {
        count += 1;
    }


    if arr[y - 1][x - 1] > 0 {
        count += 1;
    }
    if arr[y - 1][x] > 0 {
        count += 1;
    }
    if arr[y - 1][x + 1] > 0 {
        count += 1;
    }

    count
}
