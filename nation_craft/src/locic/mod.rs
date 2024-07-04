use std::default;

#[derive(Debug, Copy, Clone)]
pub struct ConstVec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> ConstVec2<T> {
    pub fn new<Z>(x: Z, y: Z) -> ConstVec2<Z> {
        return ConstVec2 {x,y}
    }
}

pub const fn newCV2<T>(x: T, y: T) -> ConstVec2<T>{
    return ConstVec2 {x,y}
}

#[derive(Debug, Copy, Clone)]
pub enum Resource_type {
    iron,
    corn,
    normalMaeterials,
    rareMaterials,
    fuel,
    wood,
    none,
}

#[derive(Debug, Copy, Clone)]
pub struct Resource {               //use as a production p. for fields or as a requirement for someting
    pub source: Resource_type,
    pub name: &'static str,
    pub value: f64,
}

impl Resource {
    pub fn get(&self) -> Resource {
        return Resource {source: self.source, name: self.name, value: self.value}
    }
}

pub const defaultResource: Resource = Resource {source: Resource_type::none, name: "default", value: 0.0};
pub const iron: Resource = Resource {source: Resource_type::iron, name: "iron", value: 10.0};
pub const corn: Resource = Resource {source: Resource_type::corn, name: "corn", value: 1.0};

pub const defaultWorldResources: [Resource; 2] = [iron, corn];

pub fn getResource<const N: usize>(r: [Resource; N]) -> [Resource; N] {
    let mut temp_list: [Resource; N] = [defaultResource; N];
    for i in 0..N {
        temp_list[i] = r.get(i).expect("REASON").get(); //without .get() because it implements copy
    }
    return temp_list;
}

#[derive(Debug, Copy, Clone)]
pub enum FieldProperties_type {}

#[derive(Debug, Copy, Clone)]
pub struct FieldProperties {
    pub property: FieldProperties_type,
    pub property_str: f64,
}

pub struct Field<const E: usize, const FP: usize, const RP: usize> {
    pub name: &'static str,
    pub points: [ConstVec2<f64>; E],
    pub val_points: i32,
    pub center: ConstVec2<f64>,
    //fieldProperties: [FieldProperties; FP],
    pub resourceProp: [Resource; RP],
    //buildings
}

pub const fp: [usize; 3] = [4,0,1];
pub const testField: Field<4,0,1> = Field {name: "test", points: [newCV2(0.0,0.0), newCV2(0.0,1.0), newCV2(1.0,1.0), newCV2(1.0,0.0)],
                                        val_points: 4, center: newCV2(0.5,0.5), resourceProp: [corn]};

pub const Map1: [Field<4,0,1>; 1] = [testField];

#[derive(Debug, Copy, Clone)]
pub struct Troup {
    id: i64,
    name: &'static str,
}

pub const Infantery1: Troup = Troup {id: 0, name: "Infantery level 1"};

//troupcluster and palyer ony has avariabel amount of troup clusters

pub struct Inventory<const RN: usize> {
    pub resources: [Resource; RN],
}

pub struct Player<const RN: usize> {
    pub inventory: Inventory<RN>,
    pub name: String,
    pub coutntry_name: String,
    pub troups: Vec<Troup>,
}

pub const defaultPlayer: Player<2> = Player {inventory: Inventory{resources: defaultWorldResources},
                         name: String::new(), coutntry_name: String::new(), troups: Vec::new()};

pub const defaultPlayerSet: [Player<2>; 1] = [defaultPlayer];

impl<const RN: usize> Player<RN> {
    fn addTroup(&mut self, t: Troup){
        self.troups.push(t);
    }
}

pub struct World<const RN: usize, const FN: usize, const M1: usize, const M2: usize, const M3: usize, const NP: usize>{
    pub avaliableResources: [Resource; RN],
    pub map: [Field<M1, M2, M3>; FN],
    pub players: [Player<RN>; NP]
}

pub fn newWorld<const N: usize, const M: usize, const M1: usize, const M2: usize, const M3: usize, const NP: usize>
    (res: [Resource; N], map: [Field<M1,M2,M3>; M], playerList: [Player<N>; NP]) -> World<N,M,M1,M2,M3,NP>{
    return World {avaliableResources: res, map: map, players: playerList};
}

