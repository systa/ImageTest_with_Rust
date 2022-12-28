/*
* This simple program was mainly written to learn basics of the Rust programming language.
* The initial version evolved as my learned more about Rust and as I got new ideas about
* the functionality.  Naturally, this means that I'm not very proud about the results.
* 
* The purpose of software is to read an image and radically limit the number of used
* color shades. The algoritm is based on five steps
*   1) a set of candidate colors is defined. See function createRefColors. The number of
*      candidate colors is parametrized (n*n*n).
*   2) Iterate through all pixels of the image:
*       Function updateDistanceData:
*        - calculate distance (see the fn "distance") to each candidate colors
*        - update counter of the closest color
*   3) Take the most popular one from the reference colors. Function popularColors
*   4) Iterate through all pixels of the image and replace with the closest in list of popular
*      colors.
*   5) If the fourth parameter exists, move the final colors with kind of simulated annealing.
*
* Notes: 
*   - Rust compiler nags me on using "snake_case". I'm not convinced.
*   - Selection between integer types (u9, i32, u32) is a bit arbitrary. One goal was to minimize
*     the casts with "as"
*   - Some data structures are almost static, but because their content depends on
*     command-line parameters they are mutable. This lead to several "unsafe blocks"
*   - Instead of code quality the attention has been put on testing the effect of different algorithms.
*
* Copyright Kari Systä, 2022.
*
* Thanks to https://lib.rs/crates/image and example for giving me a starting point.
*      
*/
use image::GenericImageView;
use std::env;
use std::cmp;
use rand::Rng;

// Not used anymore
static cols: [[i32;3];13] = [
    [100, 100, 100],
    [100, 100, 245],
    [100, 245, 100],
    [100, 245, 245],
    [245, 100, 100],
    [245, 100, 245],
    [245, 245, 100],
    [245, 245, 245],
    [220, 220, 220],
    [150, 150, 150],
 //   [50, 50, 150],
 //   [50, 150, 50],
 //   [150, 50, 50],
//    [150, 50, 150],
//    [150, 150, 50],
//    [150, 150, 50],
    [50, 50, 50],
    [180, 180, 180],
    [10, 10, 10]
];

// Value-range is actually within u8, but it is used in i32 calculation and Rust does not cast implicitly
static mut refColors:Vec<[i32;3]> = Vec::new();
static mut distSums:Vec<u32> = Vec::new();
static mut distSumsF:Vec<f64> = Vec::new();

unsafe fn createRefColors(resolution:i32) {
    let begin:i32 = 1; // 10 for non-logarithmic
    let end:i32 = 245;
    let step:i32 = ((end-begin)*10)/resolution;  // times 10 to minimize effect of the floor-bahavior of division

    for r in 0..resolution {
       for g in 0..resolution {
          for b in 0..resolution {
            let mut col:[i32;3] = [begin+(r*step)/10, begin+(g*step)/10, begin+(b*step)/10];
            if g == 0 && b == 0 && false { // debugging
                println!("COL1: {:?}", col);
            } 
            //Simple approach to add logaritmic scale
            col[0] = (((col[0] as f64).sqrt())*16.0) as i32; 
            col[1] = (((col[1] as f64).sqrt())*16.0) as i32; 
            col[2] = (((col[2] as f64).sqrt())*16.0) as i32; 
            if g == 0 && b == 0 && false { // debugging
                println!("COL2: {:?}", col);
            } 
            refColors.push(col);
            distSums.push(0);
            distSumsF.push(0.0);
          }
        }
    }
    println!("Create {:?} ref colors", refColors.len());
}

fn distance(r1:i32, r2:i32, g1:i32, g2:i32, b1:i32, b2:i32) -> f64 {
    let r:i32 = (r1-r2)*(r1-r2);
    let g:i32 = (g1-g2)*(g1-g2);
    let b:i32 = (b1-b2)*(b1-b2);
//    let dom:i32 = cmp::max(r,cmp::max(g,b));
    return ((r+g+b) as f64); //.sqrt().sqrt();
}

unsafe fn updateDistanceData(p:&mut[u8;4]) {
    let r:i32 = p[0] as i32;
    let g:i32 = p[1] as i32;
    let b:i32 = p[2] as i32;
    let mut closestDist:f64 = 9.0e42;
//    let mut distSum:f64 = 0.0;
    let mut closestIndex:i32 = -1;
    for i in 0..refColors.len() {
        let col = refColors[i];
        let dist:f64 = distance(r,col[0], g, col[1], b, col[2]);
        distSumsF[i] += dist;
        if dist < closestDist {
            closestDist = dist;
            closestIndex = i as i32;
//            println!("closest = {:?} {:?}", closestIndex, closestDist);
//            assert!(closestIndex < refColors.len() as i32, "Odd closest index {:?} {:?}", closestIndex, i);
        }
    }
//    println!("Closest = {:?} {:?}", closestIndex, closestDist);
    assert!(closestIndex >= 0, "No new closest");
    distSums[closestIndex as usize] += 1;
//    distSumsF[closestIndex as usize] += distSum;
}

// Find the n most popular colors in order of popularity
unsafe fn popularColors (n:u32) -> Vec<u32>{
    let mut sums = distSums.clone();
    let mut ret:Vec<u32> = Vec::new();
    for _i in 0..n {
        let mut popular:u32 = 0;
        let mut popJ = 0;
        for j in 0..sums.len() {
            if sums[j] > popular {
                popular = sums[j];
                popJ = j;
            }
        }
        ret.push(popJ as u32);
//        println!("{:?} {:?} : {:?}", popJ, refColors[popJ as usize], sums[popJ as usize]);
        sums[popJ as usize] = 0;
    }
    return ret.to_owned();
}

unsafe fn usePopularF (n:u32) -> Vec<u32>{
    let mut sums = distSumsF.clone();
    let mut ret:Vec<u32> = Vec::new();
    for i in 0..n {
        let mut popular:f64 = 9.9e45;
        let mut popJ = 0;
        for j in 0..sums.len() {
            if sums[j] < popular {
                popular = sums[j];
                popJ = j;
            }
        }
        ret.push(popJ as u32);
        sums[popJ as usize] = 9.9e45;
    }
    return ret.to_owned();
}

/*
 * These three are not used at the moment
fn csc(mut p: &mut[u8;4]) {
    let mut closest: u16 = 20000;
    let mut dist: u16;
    let mut closestI: usize = 0;
    let r:[i32;3] = [p[0] as i32, p[1] as i32, p[2] as i32];
    for i in 0..13 {
        dist = (((r[0]-cols[i][0])*(r[0]-cols[i][0]) +
                 (r[1]-cols[i][1])*(r[1]-cols[i][1]) +
                 (r[2]-cols[i][2])*(r[2]-cols[i][2])) as f64).sqrt() as u16;
        if dist < closest {
            closest = dist;
            closestI = i as usize;  
        }
    }
    p[0] = cols[closestI][0] as u8;
    p[1] = cols[closestI][1] as u8;
    p[2] = cols[closestI][2] as u8;
}
fn ssc(p:u8) -> u8 {
   let mut tmp: u16 = (p as u16 + 64);
   tmp = tmp/128*128;
   if tmp > 255 {
       tmp = 255;
   }
   return tmp as u8;
}
*/

// Get the closest color to 'p' in 'pop'
fn cscPop(mut p: &mut[u8;4], pop:&Vec<[i32;3]>) -> f64 {
    let mut closest: f64 = 2000000.0;
    let mut dist: f64;
    let mut closestI: usize = 0;
    let r:[i32;3] = [p[0] as i32, p[1] as i32, p[2] as i32];
    for i in 0..pop.len() {
        dist = distance(r[0], pop[i][0], r[1], pop[i][1], r[2], pop[i][2]);
        if dist < closest {
            closest = dist;
            closestI = i as usize;  
        }
    }
    p[0] = pop[closestI][0] as u8;
    p[1] = pop[closestI][1] as u8;
    p[2] = pop[closestI][2] as u8;
    return closest;
}



//Create a name for the output file
fn outputName(base:String,cf:i32, cu:u32, cr:u32) -> String {
    let mut newName:String = base;
    newName.push_str("P-");
    newName.push_str(&cf.to_string());
    newName.push_str("-");
    newName.push_str(&cu.to_string());
    newName.push_str("-");
    newName.push_str(&cr.to_string());
    newName.push_str(".jpg");
    return newName;
}


fn main() {
    let width: u32;
    let height: u32;
    let filename:String;
    let outname:String;
    let mut rounds:u32 = 0;
    let mut cFreq:i32 = 7;
    let mut cUsed:u32 = 16;
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let args:Vec<String> = env::args().collect();
    if args.len() <2 {
        println!("Usage: cargo run -- imagefile [cadidate_color_freq [used_colors [fine_tuning_rounds]]");
        return;
    }
    if args.len() > 2 {
        cFreq = args[2].clone().parse().unwrap();
        if args.len()>3 {
            cUsed = args[3].clone().parse().unwrap();
	    if args.len() > 4 {
               rounds = args[4].clone().parse().unwrap();
	    } 
        }
    }
    filename = args[1].clone();
    outname = outputName(filename.clone(), cFreq, cUsed, rounds);
    println!("Starting");
    let img = image::open(filename).unwrap();

    (width, height) = img.dimensions();

    // The color method returns the image's `ColorType`.
    unsafe {
        createRefColors(cFreq);
    }
    let mut imgbuf = image::ImageBuffer::new(width, height);
    println!("Data collecting");
    for x in 0..width {
        if x%200 == 0 && false {
            println!("x={:?}", x);
        }
        for y in 0..height {
            unsafe {
                let spixel = img.get_pixel(x, y);
                let mut data:[u8;4] = [spixel[0], spixel[1], spixel[2], spixel[3]];
                updateDistanceData(&mut data);
            }
        }
    }
    let pop;   // indexes of most used colors
    let mut popCols:Vec<[i32;3]> = Vec::new(); // actual colors,.
    unsafe {
        println!("----- NUMBER");
        pop = popularColors(cUsed);  // 'cUsed" most used colors (their indexes)
        for i in 0..pop.len() {
            println!("pop{:?} {:?} {:?} {:?} {:?}", i, pop[i], distSums[pop[i] as usize],
                     distSumsF[pop[i] as usize], refColors[pop[i] as usize]);
                     popCols.push(refColors[pop[i] as usize]);   
        }
        /*
        * Was an alternative approach that did not work well.
        println!("----- SUMS");
        pop = usePopularF(cUsed);
        for i in 0..pop.len() {
            println!("pop{:?} {:?} {:?} {:?} {:?}", i, pop[i], distSums[pop[i] as usize],
                    distSumsF[pop[i] as usize], refColors[pop[i] as usize]);

        }
        */
        println!("----- ");
    }
    println!("Fine tuning");
    let mut jump = 64;
    let mut rng = rand::thread_rng();
 // Get current total distance_
    let mut currentSum:f64 = 0.0;
    for x in 0..width {
        for y in 0..height {
            let spixel = img.get_pixel(x, y);
            let mut data:[u8;4] = [spixel[0], spixel[1], spixel[2], spixel[3]];
            currentSum += cscPop(&mut data, &popCols);        
       }
    }
    while (jump > 1 && rounds > 0) {
       println!("JUMP:{:?}", jump);
       for i in 0..popCols.len() {
           for j in 0..rounds {
                let rr:i32 = (rng.gen_range(-255..255)*jump)/256;
                let rg:i32 = (rng.gen_range(-255..255)*jump)/256;
                let rb:i32 = (rng.gen_range(-255..255)*jump)/256;
                let mut targetColor = popCols[i];
                let mut save:[i32;3] = targetColor;      
                targetColor[0] += rr;
                if targetColor[0] < 0 {targetColor[0] = 0;}
                else if targetColor[0] > 255 {targetColor[0] = 255;}
                targetColor[1] += rg;
                if targetColor[1] < 0 {targetColor[1] = 0;}
                else if targetColor[1] > 255 {targetColor[1] = 255;}
                targetColor[2] += rb;
                if targetColor[2] < 0 {targetColor[2] = 0;}
                else if targetColor[2] > 255 {targetColor[2] = 255;}
                popCols[i] = targetColor;
                let mut distanceSum:f64 = 0.0;
                for x in 0..width {
                   for y in 0..height {
                      let spixel = img.get_pixel(x, y);
                      let mut data:[u8;4] = [spixel[0], spixel[1], spixel[2], spixel[3]];
                      distanceSum += cscPop(&mut data, &popCols);  // TODO: this calculated too much -
                                                                   // should only recalculate the changed color.
                   }
                }
                if distanceSum < currentSum {
                   let diff = currentSum - distanceSum;
                   currentSum = distanceSum;
                   println!("Finetune {:?} {:?} {:?} : {:?}=>{:?}, win:{:?}",
                            jump, i, j, save, targetColor, diff);
                } else {
                   popCols[i] = save;
                }
           }
       }
       jump /= 2;
    }
    unsafe {
       for i in 0..pop.len() {
            println!("pop{:?} {:?} {:?} {:?} {:?}", i, pop[i], distSums[pop[i] as usize],
                     distSumsF[pop[i] as usize], popCols[i]);
       }
    }
    println!("Finalizing");
    for x in 0..width {
        for y in 0..height {
            let spixel = img.get_pixel(x, y);
            let dpixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgba(_data) = *dpixel;
//            *dpixel = image::Rgba([ssc(spixel[2]), ssc(spixel[0]), ssc(spixel[1]), spixel[3]]);
            let mut data:[u8;4] = [spixel[0], spixel[1], spixel[2], spixel[3]];
            if y < 256 && false {   // To test
//                let y1:u8 = ((y as f64).sqrt() * 15.9) as u8; 
                let y1:u8 = ((y *y )/256) as u8; 
                data = [y1, y1, y1, /*(x*255/width) as u8,*/ spixel[3]];
            } else {
                cscPop(&mut data, &popCols);
            }
            *dpixel = image::Rgba(data);
        }
    }
    println!("Saving");
    // Write the contents of this image to the Writer in XXX format.
    imgbuf.save(outname).unwrap();
    
}
