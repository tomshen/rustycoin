#![feature(macro_rules)]
extern crate opencl = "OpenCL#0.2";

use opencl::hl;

macro_rules! expect (
    ($test: expr, $expected: expr) => ({
            let test     = $test;
            let expected = $expected;
            if test != expected {
                fail!(format!("Test failure in {:s}: expected {:?}, got {:?}",
                              stringify!($test),
                              expected, test))
            }
        })
        )

fn main () {
    /*let src = "__kernel void test(__global int *i, long int k) {
                   *i += k;
                   }";
    let prog = ctx.create_program_from_source(src);
    prog.build(&device).unwrap();

    let k = prog.create_kernel("test");
    let v = ctx.create_buffer_from(&[1], CL_MEM_READ_WRITE);
    k.set_arg(0, &v);
    k.set_arg(1, &42);
    queue.enqueue_async_kernel(&k, 1, None, ()).wait();
    let v: ~[int] = queue.get(&v, ());

    expect!(v[0], 43);*/
    
    let platforms = hl::get_platforms();
    for p in platforms.iter() {
        println!("platform found");
        println!("platform: {}", p.name());
        let devices = p.get_devices();
        println!("got the devices!!")
        for d in devices.iter() {
            println!("device found");
            let context = d.create_context();
            let queue = context.create_command_queue(d);
            let name = d.name();
            println!("done with the device");
        }
    }
    
}
