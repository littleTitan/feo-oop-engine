use {
    feo_oop_engine::{
        scene::{
            game_object::camera::fpv_camera::FpvCamera,
        },
        scripting::{
            globals::{
                Global,
                EngineGlobals,
            },
            Scriptable, 
            swap::Swap
        }
    },
    feo_math::{
        rotation::quaternion::Quaternion,
        linear_algebra::vector3::Vector3,
        axes::Axes,
    },
    std::{
        sync::{
            Arc, 
            RwLock
        }, 
        time::Instant
    }
};

#[derive(Clone, Copy, Debug, Global)] // Global is derivable
pub struct CameraGlobals{
    #[allow(dead_code)]
    time: Instant,
    #[allow(dead_code)]
    distance_from_center: f32,
    #[allow(dead_code)]
    speed: f32,
    #[allow(dead_code)]
    rotation_axis: Vector3<f32>
    // ...
}

#[macro_rules_attribute(start_script!)]
pub async fn start<'r>(this: Arc<RwLock<FpvCamera>>, _: EngineGlobals) -> Swap {
    let this = this.clone();
    let mut this = this.write().unwrap();

    let rotation_axis = Axes::<f32>::from(this.subspace.rotation).y;
    this.set_globals(Box::new(CameraGlobals{
        time: Instant::now(),
        distance_from_center: 15.0,
        speed: 1.0,
        rotation_axis,
    }) as Box<dyn Global>).unwrap();

    Swap::None
}

#[macro_rules_attribute(frame_script!)]
pub async fn frame<'r>(this: Arc<RwLock<FpvCamera>>, _: EngineGlobals) -> Swap {
    let this = this.clone();
    let mut this = this.write().unwrap();
    let mut globals = downcast!(*this.get_globals().unwrap(), dyn Global, CameraGlobals);
    
    let elapsed = (globals.time.elapsed()); globals.time = Instant::now();
    let rotation = Quaternion::new_axis_angle(globals.rotation_axis, elapsed.as_secs() as f32 + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0) as f32 * globals.speed);


    this.subspace.center = (rotation * Quaternion(this.subspace.center.0, this.subspace.center.1, this.subspace.center.2, 0.0) * rotation.conjugate()).into();


    this.subspace.rotation = Quaternion::camera_look_at(this.subspace.center, Vector3(0.0, 0.0, 0.0));

    this.set_globals(Box::new(globals) as Box<dyn Global>).unwrap();
    
    Swap::None
}