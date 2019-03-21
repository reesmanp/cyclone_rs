var searchIndex = {};
searchIndex["assert_approx_eq"] = {"doc":"","items":[[14,"assert_approx_eq","assert_approx_eq","Asserts that two expressions are approximately (~1.0e-6) equal to each other.",null,null]],"paths":[]};
searchIndex["cyclone"] = {"doc":"","items":[[0,"constants","cyclone","",null,null],[17,"EARTH_G","cyclone::constants","Average force from gravity on Earth",null,null],[0,"core","cyclone","",null,null],[0,"vector","cyclone::core","",null,null],[3,"Vector3","cyclone::core::vector","Struct representing 3-Dimensional vectors",null,null],[12,"x","","",0,null],[12,"y","","",0,null],[12,"z","","",0,null],[12,"pad","","",0,null],[11,"new","","",0,{"inputs":[{"name":"real"},{"name":"real"},{"name":"real"}],"output":{"name":"self"}}],[11,"invert","","Takes ownership of self Returns new instance of self that is inverted",0,{"inputs":[{"name":"self"}],"output":{"name":"self"}}],[11,"invert_inplace","","Mutates self Inverts self without returning",0,{"inputs":[{"name":"self"}],"output":null}],[11,"invert_clone","","Does not take ownership of self Returns a new instance that is inverted from self",0,{"inputs":[{"name":"self"}],"output":{"name":"self"}}],[11,"magnitude","","",0,{"inputs":[{"name":"self"}],"output":{"name":"real"}}],[11,"square_magnitude","","",0,{"inputs":[{"name":"self"}],"output":{"name":"real"}}],[11,"normalize","","",0,{"inputs":[{"name":"self"}],"output":{"name":"self"}}],[11,"normalize_inplace","","",0,{"inputs":[{"name":"self"}],"output":null}],[11,"normalize_clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"self"}}],[11,"cross","","",0,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"self"}}],[11,"cross_clone","","",0,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"self"}}],[11,"dot","","",0,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"real"}}],[11,"create_orthonormal_basis","","",0,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"option"}}],[11,"find_angle","","",0,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"real"}}],[11,"default","","",0,{"inputs":[],"output":{"name":"self"}}],[11,"add","","",0,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"self"}}],[11,"sub","","",0,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"self"}}],[11,"mul","","",0,{"inputs":[{"name":"self"},{"name":"real"}],"output":{"name":"self"}}],[11,"mul","","",0,{"inputs":[{"name":"self"},{"name":"self"}],"output":{"name":"real"}}],[11,"div","","",0,{"inputs":[{"name":"self"},{"name":"real"}],"output":{"name":"self"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"self"}}],[0,"particle","cyclone::core","",null,null],[3,"Particle","cyclone::core::particle","Particle Struct",null,null],[0,"collision","","",null,null],[0,"force_generators","","",null,null],[3,"DefaultForceGenerator","cyclone::core::particle::force_generators","A basic force generator used for a simple case",null,null],[3,"DefaultGravityForceGenerator","","Basic force using Earths gravity",null,null],[3,"DefaultTemporaryForceGenerator","","Basic implementation of a force generator that has a time to live Same as `DefaultForceGenerator` except for the ttl",null,null],[3,"DefaultSpringForceGenerator","","Implements a basic spring particle force generator TODO: Test `other_particle` reference is correct",null,null],[8,"ForceGenerator","","Main trait to attach to forces",null,null],[10,"update","","",1,{"inputs":[{"name":"self"},{"generics":["vector3"],"name":"vec"}],"output":{"name":"vector3"}}],[10,"get_forces","","",1,{"inputs":[{"name":"self"}],"output":{"generics":["vector3"],"name":"vec"}}],[10,"sum_forces","","",1,{"inputs":[{"name":"self"}],"output":{"name":"vector3"}}],[10,"time_elapsed","","",1,{"inputs":[{"name":"self"},{"name":"real"}],"output":{"name":"bool"}}],[10,"clone_box","","",1,{"inputs":[{"name":"self"}],"output":{"generics":["forcegenerator"],"name":"box"}}],[11,"new","","",2,{"inputs":[{"generics":["vector3"],"name":"vec"}],"output":{"name":"self"}}],[11,"update","","",2,{"inputs":[{"name":"self"},{"generics":["vector3"],"name":"vec"}],"output":{"name":"vector3"}}],[11,"get_forces","","",2,{"inputs":[{"name":"self"}],"output":{"generics":["vector3"],"name":"vec"}}],[11,"sum_forces","","",2,{"inputs":[{"name":"self"}],"output":{"name":"vector3"}}],[11,"time_elapsed","","",2,{"inputs":[{"name":"self"},{"name":"real"}],"output":{"name":"bool"}}],[11,"clone_box","","",2,{"inputs":[{"name":"self"}],"output":{"generics":["forcegenerator"],"name":"box"}}],[11,"new","","",3,{"inputs":[],"output":{"name":"defaultforcegenerator"}}],[11,"new","","",4,{"inputs":[{"generics":["vector3"],"name":"vec"},{"name":"real"}],"output":{"name":"self"}}],[11,"update","","",4,{"inputs":[{"name":"self"},{"generics":["vector3"],"name":"vec"}],"output":{"name":"vector3"}}],[11,"get_forces","","",4,{"inputs":[{"name":"self"}],"output":{"generics":["vector3"],"name":"vec"}}],[11,"sum_forces","","",4,{"inputs":[{"name":"self"}],"output":{"name":"vector3"}}],[11,"time_elapsed","","",4,{"inputs":[{"name":"self"},{"name":"real"}],"output":{"name":"bool"}}],[11,"clone_box","","",4,{"inputs":[{"name":"self"}],"output":{"generics":["forcegenerator"],"name":"box"}}],[11,"new","","",5,{"inputs":[{"name":"real"},{"name":"real"},{"name":"particle"},{"name":"particle"}],"output":{"name":"self"}}],[11,"update","","",5,{"inputs":[{"name":"self"},{"generics":["vector3"],"name":"vec"}],"output":{"name":"vector3"}}],[11,"get_forces","","",5,{"inputs":[{"name":"self"}],"output":{"generics":["vector3"],"name":"vec"}}],[11,"sum_forces","","",5,{"inputs":[{"name":"self"}],"output":{"name":"vector3"}}],[11,"time_elapsed","","",5,{"inputs":[{"name":"self"},{"name":"real"}],"output":{"name":"bool"}}],[11,"clone_box","","",5,{"inputs":[{"name":"self"}],"output":{"generics":["forcegenerator"],"name":"box"}}],[0,"registry","cyclone::core::particle","",null,null],[3,"ParticleRegistry","cyclone::core::particle::registry","The particle storage system",null,null],[12,"particles","","",6,null],[11,"new","","",6,{"inputs":[],"output":{"name":"self"}}],[11,"count","","",6,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"exists","","",6,{"inputs":[{"name":"self"},{"name":"string"}],"output":{"name":"bool"}}],[11,"get_particle","","",6,{"inputs":[{"name":"self"},{"name":"string"}],"output":{"generics":["particle"],"name":"option"}}],[11,"add_particle","","",6,{"inputs":[{"name":"self"},{"name":"string"},{"name":"particle"}],"output":null}],[11,"remove_particle","","",6,{"inputs":[{"name":"self"},{"name":"string"}],"output":null}],[11,"update","","",6,{"inputs":[{"name":"self"},{"name":"real"}],"output":null}],[11,"clone","cyclone::core::particle","",7,{"inputs":[{"name":"self"}],"output":{"name":"particle"}}],[11,"new","","",7,{"inputs":[{"name":"vector3"},{"name":"vector3"},{"name":"vector3"},{"generics":["real"],"name":"option"}],"output":{"name":"self"}}],[11,"get_position","","Getters",7,{"inputs":[{"name":"self"}],"output":{"name":"vector3"}}],[11,"get_velocity","","",7,{"inputs":[{"name":"self"}],"output":{"name":"vector3"}}],[11,"get_acceleration","","",7,{"inputs":[{"name":"self"}],"output":{"name":"vector3"}}],[11,"get_mass","","",7,{"inputs":[{"name":"self"}],"output":{"name":"real"}}],[11,"get_force","","",7,{"inputs":[{"name":"self"},{"name":"string"}],"output":{"generics":["vector3"],"name":"option"}}],[11,"get_forces","","",7,{"inputs":[{"name":"self"}],"output":{"generics":["vec"],"name":"option"}}],[11,"get_momentum","","",7,{"inputs":[{"name":"self"}],"output":{"name":"vector3"}}],[11,"set_position","","Setters",7,{"inputs":[{"name":"self"},{"name":"vector3"}],"output":null}],[11,"set_velocity","","",7,{"inputs":[{"name":"self"},{"name":"vector3"}],"output":null}],[11,"set_acceleration","","",7,{"inputs":[{"name":"self"},{"name":"vector3"}],"output":null}],[11,"set_mass","","",7,{"inputs":[{"name":"self"},{"generics":["real"],"name":"option"}],"output":null}],[11,"set_force","","",7,{"inputs":[{"name":"self"},{"name":"string"},{"name":"t"}],"output":null}],[11,"update","","Updaters",7,{"inputs":[{"name":"self"},{"name":"real"}],"output":null}],[11,"update_position","","",7,{"inputs":[{"name":"self"},{"name":"vector3"}],"output":null}],[11,"update_velocity","","",7,{"inputs":[{"name":"self"},{"name":"vector3"}],"output":null}],[11,"sum_forces","","Utility Functions",7,{"inputs":[{"name":"self"}],"output":{"name":"vector3"}}],[0,"types","cyclone","",null,null],[6,"Real","cyclone::types","Real type using 64 bits Compiled by default",null,null],[11,"mul","","",8,{"inputs":[{"name":"self"},{"name":"vector3"}],"output":{"name":"vector3"}}],[11,"mul","","",8,{"inputs":[{"name":"self"},{"name":"vector3"}],"output":{"name":"vector3"}}]],"paths":[[3,"Vector3"],[8,"ForceGenerator"],[3,"DefaultForceGenerator"],[3,"DefaultGravityForceGenerator"],[3,"DefaultTemporaryForceGenerator"],[3,"DefaultSpringForceGenerator"],[3,"ParticleRegistry"],[3,"Particle"],[6,"Real"]]};
initSearch(searchIndex);
