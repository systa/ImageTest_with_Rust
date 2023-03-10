/*
* This simple program was mainly written to learn basics of the Rust programming language.
* The initial version evolved as my learned more about Rust and as I got new ideas about
* the functionality.  Naturally, this means that I'm not very proud about all the results.
* 
* For more info, see README.md.
*
* Copyright Kari Systä, 2022.
*      
*/
use image::GenericImageView;
use std::env;
use std::cmp;
use rand::Rng;
// use int_hash::IntHashSet;
use std::collections::HashSet;


// Value-range is actually within u8, but it is used in i32 calculation and Rust does not cast implicitly
static mut refColors:Vec<[i32;3]> = Vec::new();
static mut distSums:Vec<u32> = Vec::new();
static mut distSumsF:Vec<u64> = Vec::new();

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
            distSumsF.push(0);
          }
        }
    }
    println!("Create {:?} ref colors", refColors.len());
}

fn distance(r1:i32, r2:i32, g1:i32, g2:i32, b1:i32, b2:i32) -> u32 {
    let r:i32 = (r1-r2)*(r1-r2);
    let g:i32 = (g1-g2)*(g1-g2);
    let b:i32 = (b1-b2)*(b1-b2);
    return (r+g+b) as u32; 
}

unsafe fn updateDistanceData(p:&mut[u8;4]) {
    let r:i32 = p[0] as i32;
    let g:i32 = p[1] as i32;
    let b:i32 = p[2] as i32;
    let mut closestDist:u32 = 2000000000;
    let mut closestIndex:i32 = -1;
    for i in 0..refColors.len() {
        let col = refColors[i];
        let dist:u32 = distance(r,col[0], g, col[1], b, col[2]);
        distSumsF[i] += dist as u64;
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
        let mut popular:u64 = 2000000000;
        let mut popJ = 0;
        for j in 0..sums.len() {
            if sums[j] < popular {
                popular = sums[j];
                popJ = j;
            }
        }
        ret.push(popJ as u32);
        sums[popJ as usize] = 2000000000; // to mark as taken
    }
    return ret.to_owned();
}


// Get the closest color to 'p' in 'pop'
fn cscPop(mut p: &mut[u8;4], pop:&Vec<[i32;3]>) -> u32 {
    let mut closest: u32 = 2000000000;
    let mut dist: u32;
    let mut closestI: usize = 0;
    let r:[i32;3] = [p[0] as i32, p[1] as i32, p[2] as i32];
//    let r:[i32;3] = p as [i32;3];
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

// Get the closest color to 'p' in 'pop' - optimized for finetuning
// TODO check if parameters should be mutable
fn cscPop1(mut r: &mut[i32;3], pop:&Vec<[i32;3]>) -> u32 {
    let mut closest: u32 = 2000000000;
    let mut dist: u32;
    let mut closestI: usize = 0;
    for i in 0..pop.len() {
        dist = distance(r[0], pop[i][0], r[1], pop[i][1], r[2], pop[i][2]);
        if dist < closest {
            closest = dist;
            closestI = i as usize;  
        }
    }
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
        println!("----- ");
    }
    println!("Fine tuning");
    let mut jump = 64;
    let mut rng = rand::thread_rng();
 // Get current total distance_
    let mut currentSum:u64 = 0;
    for x in 0..width {
        for y in 0..height {
            let spixel = img.get_pixel(x, y);
            let mut data:[i32;3] = [spixel[0] as i32, spixel[1] as i32, spixel[2] as i32];
            currentSum += cscPop1(&mut data, &popCols) as u64;        
       }
    }
//    let mut tried:IntHashSet<i32> = IntHashSet::default();; 

    while jump > 1 && rounds > 0 {
       println!("JUMP:{:?}", jump);
       let oldCols = popCols.clone(); 
       let mut oldc = 0;
       let mut newc = 0;
       for i in 0..popCols.len() {
            let mut tried:HashSet<i32> = HashSet::new();
            let mut key;
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
//                key = (targetColor[0]*256*256+targetColor[1]*256+targetColor[2])*(popCols.len() as i32) + i as i32;
                key = targetColor[0]*256*256+targetColor[1]*256+targetColor[2];
                if ! tried.contains(&key) {
                    tried.insert(key);
                    popCols[i] = targetColor;
                    let mut distanceSum:u64 = 0;
                    for x in 0..width {
                        for y in 0..height {
                            let spixel = img.get_pixel(x, y);
                            let mut data:[i32;3] = [spixel[0] as i32, spixel[1] as i32, spixel[2] as i32];
                            distanceSum += cscPop1(&mut data, &popCols) as u64;  // TODO: this calculates too much -
                                                                   // should only recalculate for the changed color.
                        }
                    }
                    if distanceSum < currentSum {
                       let diff = currentSum - distanceSum;
                       currentSum = distanceSum;
                       /*
                       println!("Finetune {:?} {:?} {:?} : {:?}=>{:?}={:?},  win:{:?}, totDistance:{:?}",
                                jump, i, j, save, targetColor,
                                [targetColor[0]-save[0], targetColor[1]-save[1], targetColor[2]-save[2]],
                                 diff, currentSum);
                                 */
                    } else {
                        popCols[i] = save;
                    }
                    newc += 1;
                } else {
                    oldc += 1;
                }
           }
       }
       for i in 0..popCols.len() {
        println!("{:?}: {:?} = {:?}", i, oldCols[i], popCols[i]);
       }
       println!("new: {:?}, tried: {:?}", newc, oldc);
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
            cscPop(&mut data, &popCols);
            *dpixel = image::Rgba(data);
        }
    }
    println!("Saving");
    // Write the contents of this image to the Writer in XXX format.
    imgbuf.save(outname).unwrap();
    
}
