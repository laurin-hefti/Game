#[derive(Debug, Copy, Clone)]
struct ConstVec2<T> {
    x: T,
    y: T,
}

impl<T> ConstVec2<T> {
    fn new<Z>(x: Z, y: Z) -> ConstVec2<Z> {
        return ConstVec2 {x,y}
    }
}

const fn newCV2<T>(x: T, y: T) -> ConstVec2<T>{
    return ConstVec2 {x,y}
}

#[derive(Debug, Copy, Clone)]
enum Resource_type {
    iron,
    corn,
    normalMaeterials,
    rareMaterials,
    fuel,
    wood,
    none,
}

#[derive(Debug, Copy, Clone)]
struct Resource {               //use as a production p. for fields or as a requirement for someting
    source: Resource_type,
    name: &'static str,
    value: f64,
}

impl Resource {
    fn get(&self) -> Resource {
        return Resource {source: self.source, name: self.name, value: self.value}
    }
}

const defaultResource: Resource = Resource {source: Resource_type::none, name: "default", value: 0.0};
const iron: Resource = Resource {source: Resource_type::iron, name: "iron", value: 10.0};
const corn: Resource = Resource {source: Resource_type::corn, name: "corn", value: 1.0};

const defaultWorldResources: [Resource; 2] = [iron, corn];

fn getResource<const N: usize>(r: [Resource; N]) -> [Resource; N] {
    let mut temp_list: [Resource; N] = [defaultResource; N];
    for i in 0..N {
        temp_list[i] = r.get(i).expect("REASON").get(); //without .get() because it implements copy
    }
    return temp_list;
}

enum FieldProperties_type {}

struct FieldProperties {
    property: FieldProperties_type,
    property_str: f64,
}

struct Field<const E: usize, const FP: usize, const RP: usize> {
    name: &'static str,
    points: [ConstVec2<f64>; E],
    val_points: i32,
    center: ConstVec2<f64>,
    //fieldProperties: [FieldProperties; FP],
    resourceProp: [Resource; RP],
    //buildings
}

const fp: [usize; 3] = [4,0,1];
const testField: Field<4,0,1> = Field {name: "test", points: [newCV2(0.0,0.0), newCV2(0.0,1.0), newCV2(1.0,1.0), newCV2(1.0,0.0)],
                                        val_points: 4, center: newCV2(0.5,0.5), resourceProp: [corn]};

const Map1: [Field<4,0,1>; 1] = [testField];

struct Troup {
    
}

struct Inventory<const RN: usize> {
    resources: [Resource; RN],
}

struct Player {
    inventory: Inventory,
    name: String,
    coutntry_name: String,
    //troups: dynamicArray
}

struct World<const RN: usize, const FN: usize, const M1: usize, const M2: usize, const M3: usize>{
    avaliableResources: [Resource; RN],
    map: [Field<M1, M2, M3>; FN],
}

fn newWorld<const N: usize, const M: usize, const M1: usize, const M2: usize, const M3: usize>
    (res: [Resource; N], map: [Field<M1,M2,M3>; M]) -> World<N,M,M1,M2,M3>{
    return World {avaliableResources: res, map: map};
}

fn main () {
    let w0: World<2,1,4,0,1> = newWorld(getResource(defaultWorldResources),Map1);
    //let w1 = initWorld![iron];
    println!("{}", w0.avaliableResources[1].name);
}
