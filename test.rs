#![feature(macro_rules)]
extern crate opencl = "OpenCL#0.2";

use opencl::hl;
use opencl::hl::{GPU, EventList};
use opencl::CL::CL_MEM_READ_WRITE;

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
  
  let platforms = hl::get_platforms();
  for p in platforms.iter() {
    println!("platform found");
    println!("platform: {}", p.name());
    let devices = p.get_devices_by_types(&[GPU]);
    println!("got the devices!!")
      for d in devices.iter() {
        println!("device found");
        let ctx = d.create_context();
        let queue = ctx.create_command_queue(d);
        let name = d.name();
        let src = "__kernel void test(__global int *i, long int k) {
          *i += k;
        }";
        let prog = ctx.create_program_from_source(src);
        prog.build(d).unwrap();

        let k = prog.create_kernel("test");
        let v = ctx.create_buffer_from(&[1], CL_MEM_READ_WRITE);
        k.set_arg(0, &v);
        k.set_arg(1, &42);
        queue.enqueue_async_kernel(&k, 1, None, ()).wait();
        let v: Vec<int> = queue.get(&v, ());

        expect!(*v.get(0), 43);

        println!("done with the device");
      }
  }

}
