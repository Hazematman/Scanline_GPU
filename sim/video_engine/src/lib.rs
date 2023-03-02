#[cfg(test)]
mod tests {
    mod modules {
        include!(concat!(env!("OUT_DIR"), "/modules.rs"));
    }

    use modules::*;

    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    #[test]
    fn screen_test() {
        let mut video_engine = VideoEngine::new();

        video_engine.reset();
        video_engine.prop();

        let path = Path::new("out.ppm");
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all("P3\n640 480\n255\n".as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }

        for y in 0..525 {
            for x in 0..800 {
                video_engine.prop();
                if x < 640 && y < 480 {
                    match file.write_all(format!("{} {} {} ", video_engine.r, video_engine.g, video_engine.b).as_bytes()) {
                        Err(why) => panic!("couldn't write to {}: {}", display, why),
                        Ok(_) => println!("successfully wrote to {}", display),
                    }
                }
                if x > 448 {
                    //println!("Clear value {}", video_engine.debug_clear_index);
                }
                if video_engine.r != 0 {
                    println!("Pixel Value {} {} = {} {} {}", x, y, video_engine.r, video_engine.g, video_engine.b);
                    println!("\tFrom index {}", video_engine.debug_visible_index);
                }
                if video_engine.debug_write {
                    //println!("Write at {} {} {}={}", x, y, video_engine.debug_index, video_engine.debug_pixel);
                }
                video_engine.posedge_clk();
            }
        }
    }
}
