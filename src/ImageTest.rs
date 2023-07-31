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
//use std::cmp;
use rand::Rng;
// use int_hash::IntHashSet;
use std::collections::HashSet;


// Value-range is actually within u8, but it is used in i32 calculation and Rust does not cast implicitly
static mut REF_COLORS:Vec<[i32;3]> = Vec::new();
static mut DIST_SUMS:Vec<u32> = Vec::new();
static mut DIST_SUMS_F:Vec<u64> = Vec::new();

unsafe fn create_ref_colors(resolution:i32) {
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
            REF_COLORS.push(col);
            DIST_SUMS.push(0);
            DIST_SUMS_F.push(0);
          }
        }
    }
    println!("Create {:?} ref colors", REF_COLORS.len());
}

fn distance(r1:i32, r2:i32, g1:i32, g2:i32, b1:i32, b2:i32) -> u32 {
    let r:i32 = (r1-r2)*(r1-r2);
    let g:i32 = (g1-g2)*(g1-g2);
    let b:i32 = (b1-b2)*(b1-b2);
    return (r+g+b) as u32; 
}

unsafe fn update_distance_data(p:&mut[u8;4]) {
    let r:i32 = p[0] as i32;
    let g:i32 = p[1] as i32;
    let b:i32 = p[2] as i32;
    let mut closest_dist:u32 = 2000000000;
    let mut closest_index:i32 = -1;
    for i in 0..REF_COLORS.len() {
        let col = REF_COLORS[i];
        let dist:u32 = distance(r,col[0], g, col[1], b, col[2]);
        DIST_SUMS_F[i] += dist as u64;
        if dist < closest_dist {
            closest_dist = dist;
            closest_index = i as i32;
//            println!("closest = {:?} {:?}", closest_index, closest_dist);
//            assert!(closest_index < REF_COLORS.len() as i32, "Odd closest index {:?} {:?}", closest_index, i);
        }
    }
//    println!("Closest = {:?} {:?}", closest_index, closest_dist);
    assert!(closest_index >= 0, "No new closest");
    DIST_SUMS[closest_index as usize] += 1;
//    DIST_SUMS_F[closest_index as usize] += distSum;
}

// Find the n most popular colors in order of popularity
unsafe fn popular_colors (n:u32) -> Vec<u32>{
    let mut sums = DIST_SUMS.clone();
    let mut ret:Vec<u32> = Vec::new();
    for _i in 0..n {
        let mut popular:u32 = 0;
        let mut pop_j = 0;
        for j in 0..sums.len() {
            if sums[j] > popular {
                popular = sums[j];
                pop_j = j;
            }
        }
        ret.push(pop_j as u32);
//        println!("{:?} {:?} : {:?}", pop_j, REF_COLORS[pop_j as usize], sums[pop_j as usize]);
        sums[pop_j as usize] = 0;
    }
    return ret.to_owned();
}

//OBS! Not used at the moment
unsafe fn use_popular_f (n:u32) -> Vec<u32>{
    let mut sums = DIST_SUMS_F.clone();
    let mut ret:Vec<u32> = Vec::new();
    for _i in 0..n {
        let mut popular:u64 = 2000000000;
        let mut pop_j = 0;
        for j in 0..sums.len() {
            if sums[j] < popular {
                popular = sums[j];
                pop_j = j;
            }
        }
        ret.push(pop_j as u32);
        sums[pop_j as usize] = 2000000000; // to mark as taken
    }
    return ret.to_owned();
}


// Get the closest color to 'p' in 'pop'; returns the index
fn csc_pop(p: &mut[u8;4], pop:&Vec<[i32;3]>) -> usize {
    let mut closest: u32 = 2000000000;
    let mut dist: u32;
    let mut closest_i: usize = 0;
    let r:[i32;3] = [p[0] as i32, p[1] as i32, p[2] as i32];
//    let r:[i32;3] = p as [i32;3];
    for i in 0..pop.len() {
        dist = distance(r[0], pop[i][0], r[1], pop[i][1], r[2], pop[i][2]);
        if dist < closest {
            closest = dist;
            closest_i = i as usize;  
        }
    }
    p[0] = pop[closest_i][0] as u8;
    p[1] = pop[closest_i][1] as u8;
    p[2] = pop[closest_i][2] as u8;
    return closest_i;
}

// Get the closest color to 'p' in 'pop' - optimized for finetuning
// TODO check if parameters should be mutable
fn csc_pop1(r: &mut[i32;3], pop:&Vec<[i32;3]>) -> u32 {
    let mut closest: u32 = 2000000000;
    let mut dist: u32;
//    let mut closest_i: usize = 0;
    for i in 0..pop.len() {
        dist = distance(r[0], pop[i][0], r[1], pop[i][1], r[2], pop[i][2]);
        if dist < closest {
            closest = dist;
//            closest_i = i as usize;  
        }
    }
    return closest;
}




//Create a name for the output file
fn output_name(base:String,cf:i32, cu:u32, cr:u32) -> String {
    let mut new_name:String = base;
    new_name.push_str("P-");
    new_name.push_str(&cf.to_string());
    new_name.push_str("-");
    new_name.push_str(&cu.to_string());
    new_name.push_str("-");
    new_name.push_str(&cr.to_string());
    new_name.push_str(".jpg");
    return new_name;
}


fn main() {
    let width: u32;
    let height: u32;
    let filename:String;
    let outname:String;
    let mut rounds:u32 = 0;
    let mut color_freq:i32 = 7;
    let mut colors_used:u32 = 16;
    let mut layers:bool = false;
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let mut args:Vec<String> = env::args().collect();
    if args.len() <2 {
        println!("Usage: cargo run -- imagefile ['-layers'] [cadidate_color_freq [used_colors [fine_tuning_rounds]]");
        return;
    }
    if args.len() > 2 && args[2] == "-layers" {
        layers = true;
	args.remove(2);
    }
    if args.len() > 2 {
        color_freq = args[2].clone().parse().unwrap();
        if args.len()>3 {
            colors_used = args[3].clone().parse().unwrap();
	    if args.len() > 4 {
               rounds = args[4].clone().parse().unwrap();
	    } 
        }
    }
    filename = args[1].clone();
    outname = output_name(filename.clone(), color_freq, colors_used, rounds);
    println!("Starting");
    let img = image::open(filename).unwrap();

    (width, height) = img.dimensions();

    // The color method returns the image's `ColorType`.
    unsafe {
        create_ref_colors(color_freq);
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
                update_distance_data(&mut data);
            }
        }
    }
    let pop;   // indexes of most used colors
    let mut pop_cols:Vec<[i32;3]> = Vec::new(); // actual colors,.
    unsafe {
        println!("----- NUMBER");
        pop = popular_colors(colors_used);  // 'colors_used" most used colors (their indexes)
        for i in 0..pop.len() {
            println!("pop{:?} {:?} {:?} {:?} {:?}", i, pop[i], DIST_SUMS[pop[i] as usize],
                     DIST_SUMS_F[pop[i] as usize], REF_COLORS[pop[i] as usize]);
                     pop_cols.push(REF_COLORS[pop[i] as usize]);   
        }
        println!("----- ");
    }
    println!("Fine tuning");
    let mut jump = 64;
    let mut rng = rand::thread_rng();
 // Get current total distance_
    let mut current_sum:u64 = 0;
    for x in 0..width {
        for y in 0..height {
            let spixel = img.get_pixel(x, y);
            let mut data:[i32;3] = [spixel[0] as i32, spixel[1] as i32, spixel[2] as i32];
            current_sum += csc_pop1(&mut data, &pop_cols) as u64;        
       }
    }
//    let mut tried:IntHashSet<i32> = IntHashSet::default();; 

    while jump > 1 && rounds > 0 {
       println!("JUMP:{:?}", jump);
       let old_cols = pop_cols.clone(); 
       let mut oldc = 0;
       let mut newc = 0;
       for i in 0..pop_cols.len() {
            let mut tried:HashSet<i32> = HashSet::new();
            let mut key;
           for _j in 0..rounds {
                let rr:i32 = (rng.gen_range(-255..255)*jump)/256;
                let rg:i32 = (rng.gen_range(-255..255)*jump)/256;
                let rb:i32 = (rng.gen_range(-255..255)*jump)/256;
                let mut target_color = pop_cols[i];
                let save:[i32;3] = target_color;      
                target_color[0] += rr;
                if target_color[0] < 0 {target_color[0] = 0;}
                else if target_color[0] > 255 {target_color[0] = 255;}
                target_color[1] += rg;
                if target_color[1] < 0 {target_color[1] = 0;}
                else if target_color[1] > 255 {target_color[1] = 255;}
                target_color[2] += rb;
                if target_color[2] < 0 {target_color[2] = 0;}
                else if target_color[2] > 255 {target_color[2] = 255;}
//                key = (target_color[0]*256*256+target_color[1]*256+target_color[2])*(pop_cols.len() as i32) + i as i32;
                key = target_color[0]*256*256+target_color[1]*256+target_color[2];
                if ! tried.contains(&key) {
                    tried.insert(key);
                    pop_cols[i] = target_color;
                    let mut distance_sum:u64 = 0;
                    for x in 0..width {
                        for y in 0..height {
                            let spixel = img.get_pixel(x, y);
                            let mut data:[i32;3] = [spixel[0] as i32, spixel[1] as i32, spixel[2] as i32];
                            distance_sum += csc_pop1(&mut data, &pop_cols) as u64;  // TODO: this calculates too much -
                                                                   // should only recalculate for the changed color.
                        }
                    }
                    if distance_sum < current_sum {
//                       let diff = current_sum - distance_sum;
                       /*
                       println!("Finetune {:?} {:?} {:?} : {:?}=>{:?}={:?},  win:{:?}, totDistance:{:?}",
                                jump, i, j, save, target_color,
                                [target_color[0]-save[0], target_color[1]-save[1], target_color[2]-save[2]],
                                 diff, current_sum);
                                 */
                       current_sum = distance_sum;
                    } else {
                        pop_cols[i] = save;
                    }
                    newc += 1;
                } else {
                    oldc += 1;
                }
           }
       }
       for i in 0..pop_cols.len() {
        println!("{:?}: {:?} = {:?}", i, old_cols[i], pop_cols[i]);
       }
       println!("new: {:?}, tried: {:?}", newc, oldc);
       jump /= 2;
    }
    unsafe {
       for i in 0..pop.len() {
            println!("pop{:?} {:?} {:?} {:?} {:?}", i, pop[i], DIST_SUMS[pop[i] as usize],
                     DIST_SUMS_F[pop[i] as usize], pop_cols[i]);
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
            csc_pop(&mut data, &pop_cols);
//	    if x > 100 && x < 103 && y > 100 && y < 103 {
//	       println!("{:?} at {:?},{:?} - {:?},{:?},{:?},{:?}", i, x, y, data[0], data[1], data[2], data[3]); 
//	    }
            *dpixel = image::Rgba(data);
        }
    }
    println!("Saving");
    // Write the contents of this image to the Writer in XXX format.
    imgbuf.save(outname.clone()).unwrap();

// Write separate layer-files
   if layers {
     println!("Creating layer files");
     for i in 0..pop_cols.len() {
       let mut layer_name = outname.clone();
        layer_name.push_str(&i.to_string());
        layer_name.push_str(".png");
    	let mut layerbuf = image::ImageBuffer::new(width, height);
    	for x in 0..width {
            for y in 0..height {
            	let spixel = img.get_pixel(x, y);
            	let dpixel = layerbuf.get_pixel_mut(x, y);
            	let image::Rgba(_data) = *dpixel;
            	let mut data:[u8;4] = [spixel[0], spixel[1], spixel[2], spixel[3]];
            	let j = csc_pop(&mut data, &pop_cols);
//	    	if x > 100 && x < 103 && y > 100 && y < 103 {
//	       	   println!("{:?} at {:?},{:?} - {:?},{:?},{:?},{:?}", j, x, y, data[0], data[1], data[2], data[3]); 
//	    	}
	    	if j == i {
//	       println!("5 at {:?},{:?} - {:?},{:?},{:?},{:?}", x, y, data[0], data[1], data[2], data[3]); 
	           *dpixel = image::Rgba(data);
	        } else if (x == 0 && y == 0) || (x == width-1 && y == height-1) {
		  // This is ugly. To ensure that "copy" in photoshot takes the whole image
		  // we need to ensure that the image ranges to all corners.
		  data = [spixel[0], spixel[1], spixel[2], spixel[3]];
		  println!("{:?}", data);
	          *dpixel = image::Rgba(data);
		}
	    }
	}
    	layerbuf.save(layer_name).unwrap();
     }
   }
}
