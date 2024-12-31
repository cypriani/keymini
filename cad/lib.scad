use <key.scad>
use <utils.scad>

$fs=0.1;
$fa=1;

pcb_depth=5.4-1.6;
electronic_pocket_depth=pcb_depth-1.4;

nb_cols=5;
nb_rows=3;
nb_thumbs=3;
inter_switch_x=18;
inter_switch_y=17;
deltas=[0,2,6,2,-10];// column stagger
hand_angle=15;
top=10;

module one_side_key_placement(side) {
    rotate([0,0,side * hand_angle]) {
          for (j=[0:nb_cols-1]) {
               translate([side*(j+0.5)*inter_switch_x, deltas[j], 0]) {
                    for (i=[0.5:nb_rows]) {
                         translate([0, -i*inter_switch_y, 0]) children();
                    }
               }
          }
          min_deltas=min([for (i=[0:nb_thumbs-1]) deltas[i]]);
          for (j=[0:nb_thumbs-2]) {
              delta=min(deltas[j], deltas[j+1]);
              translate([side*(j+1)*inter_switch_x, -(0.5+nb_rows)*inter_switch_y + delta]) children();
          }
     }
}

module extreme_thumb_key_placement(side) {
    rotate([0,0,side * hand_angle])
        translate([0, -(0.5+nb_rows)*inter_switch_y + deltas[0]])
            children();
}

module key_placement() {
     for (side=[-1,1]) {
          one_side_key_placement(side) children();
          extreme_thumb_key_placement(side) children();
     }
}

module outline() {
    for (side=[-1,1]) {
        hull() one_side_key_placement(side)
            square([inter_switch_x, inter_switch_y], center=true);
    }
    hull() {
        translate([0, top/2]) square([6*inter_switch_x, top], center=true);
        for (side=[-1,1])
            extreme_thumb_key_placement(side)
                square([inter_switch_x, inter_switch_y], center=true);
    }
}

module sod323() {
    color([0.8,0.8,0.8]) linear_extrude(0.15) square([2.7,0.35], center=true);
    color([0.1,0.1,0.1]) linear_extrude(1.1) square([1.8,1.4], center=true);
}

module smd0805() {
    linear_extrude(1) square([2,1.25], center=true);
}

module diode_placement() {
    key_placement() translate([0,4.7]) children();
}

module pcb() {
    translate([0,0,-pcb_depth]) {
        // FR4
        color([0,0.5,0]) {
            difference() {
                translate([0,0,-1.6]) linear_extrude(1.6, convexity=4) outline();
                key_placement() {
                    cylinder(d=3.429, h=4, center=true);
                    for (i=[-5.5,5.5]) translate([i,0]) cylinder(d=1.7018, h=4, center=true);
                    translate([0,-5.9]) cylinder(d=1.27, h=4, center=true);
                    translate([-5,-3.8]) cylinder(d=1.27, h=4, center=true);
                }
            }
        }
       // USB-C connector
        color([0.8,0.8,0.8]) translate([0,top,3.2/2]) rotate([90,0,0]) linear_extrude(7.4)
            rounded_square([9, 3.2], r=1.2, center=true);
        // MCU
        translate([0,-41]) {
            color([0.1,0.1,0.1]) linear_extrude(1.6) rounded_square([7,7], r=0.75, center=true);
            color([0.8,0.8,0.8]) linear_extrude(0.15) for (i=[-2.75:0.5:2.75]) {
                translate([i,0]) square([0.2,9], center=true);
                translate([0,i]) square([9,0.2], center=true);
            }
        }
        // LDO
        translate([0,-28]) {
            color([0.1,0.1,0.1]) linear_extrude(1) square([2.9,1.3], center=true);
            color([0.8,0.8,0.8]) linear_extrude(0.15) {
                translate([0,0.6]) square([0.4,1.2], center=true);
                translate([0.95,-0.6]) square([0.4,1.2], center=true);
                translate([-0.95,-0.6]) square([0.4,1.2], center=true);
            }
        }
        // resistors
        color([0.1,0.1,0.1]) {
            for (i=[-2,2]) translate([i,-19]) rotate([0,0,90]) smd0805();
            translate([0,-50]) smd0805();
        }
        // capacitors
        color([0.8,0.7,0.5]) {
            translate([0,-24]) smd0805();
            translate([0,-32]) smd0805();
            for (i=[-6,6]) {
                translate([i,-35]) smd0805();
                translate([i,-47]) smd0805();
            }
        }
        // diodes
        diode_placement() sod323();
    }
}

module plate() {
    difference() {
        outline();
        key_placement() square([14, 14], center=true);
        tolerance=0.1;
        translate([0,top]) square([9+2*tolerance, (7.4+tolerance)*2], center=true);
        //translate([(9+2*tolerance)/2, top-((7.4+tolerance)*2)/2]) rotate(45) translate([0, 1.5]) circle(1.5);
    }
}

module fat_plate(pocket_rounding=0.5, visible_components=false) {
    translate([0,0,-pcb_depth]) {
        difference() {
            linear_extrude(pcb_depth, convexity=4) plate();

            // Extended key holes
            key_placement() translate([0,0,-1]) linear_extrude(pcb_depth+1-1.2)
                square([15, 14], center=true);

            // Center pocket
            translate([0,0,-1]) linear_extrude(visible_components?pcb_depth+2:electronic_pocket_depth+1) {
                rounded(pocket_rounding) intersection() {
                    s=nb_rows*17;
                    rotate([0, 0, -hand_angle]) translate([0,-s]) square([s, s]);
                    mirror([1,0]) rotate([0, 0, -hand_angle]) translate([0,-s]) square([s, s]);
                }
            }

            // USB-C connector pocket
            translate([0,top-1.5-7.5/2,-1]) linear_extrude(2)
                rounded_square([9+2, 7.5], r=1.5, center=true);
        }
    }
}

module back() {
    color([0.2,0.2,0.2]) translate([0,0,-pcb_depth-1.6-0.6]) linear_extrude(0.6) difference() {
        outline();
        translate([0,-48]) rounded_square([16,3], center=true, r=1.49);
    }
}

color([0.5,0.5,0.5]) fat_plate(pocket_rounding=5, visible_components=true);
pcb();
//back();
color([1,1,1,0.8]) key_placement() switch();
color([0.9,0.9,0.9]) key_placement() keycap();
