use std::env;
use std::slice::Chunks;
use std::f32::consts;
use libm;

fn main() {
    let mut args = env::args();
    let arg1 = args.nth(1).unwrap_or(String::new()).parse::<i32>(); // conv size
    let arg2 = args.nth(0).unwrap_or(String::new()).parse::<u32>(); // radius of border

    if arg1.is_err() || arg2.is_err() {
        println!("Usage: kernel_generator <conv_size> ");
        return;
    }

    let size = arg1.unwrap();
    let radius = arg2.unwrap() as f32;
    let radius22f = radius*radius;

    let total = (0..size).fold(0, |acc,i| acc + i) * size;
    
    println!("static const int kSize {};", total + 1);
    println!("static const float2 cKernel[kSize] = {{");
    println!("  float2(0,0),");

    for i in -size..size {
        for j in -size..size {
            let a = i as f32;
            let b = j as f32; 
            let coeffX = a * ((-(a*a+b*b) / radius22f)/radius).exp();
            let coeffY = b * ((-(b*b+a*a) / radius22f)/radius).exp();
            println!("    float2({},{}),", coeffX, coeffY);
        }
    }


    println!("}};");


    // let lines = "I’d just like to interject for a moment. What you’re refering to as Linux, is in fact, GNU/Linux, or as I’ve recently taken to calling it, GNU plus Linux. Linux is not an operating system unto itself, but rather another free component of a fully functioning GNU system made useful by the GNU corelibs, shell utilities and vital system components comprising a full OS as defined by POSIX. Many computer users run a modified version of the GNU system every day, without realizing it. Through a peculiar turn of events, the version of GNU which is widely used today is often called \"Linux\", and many of its users are not aware that it is basically the GNU system, developed by the GNU Project. There really is a Linux, and these people are using it, but it is just a part of the system they use. Linux is the kernel: the program in the system that allocates the machine's resources to the other programs that you run. The kernel is an essential part of an operating system, but useless by itself; it can only function in the context of a complete operating system. Linux is normally used in combination with the GNU operating system: the whole system is basically GNU with Linux added, or GNU/Linux. All the so-called \"Linux\" distributions are really distributions of GNU/Linux.";
    // let chunks = lines.chars()
    //                 .collect::<Vec<_>>()
    //                 .chunks(100)
    //                 .map(|x| x.iter().collect())
    //                 .collect::<Vec<String>>();
    // println!("{:#?}", chunks);

    // // println!("{}",chunks.iter().count());

}