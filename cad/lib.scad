use <key.scad>
use <utils.scad>

$fs=0.5;
$fa=1;

pcb_depth=2.2+1;
plate_with_interior=true;

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

module outline(with_interior=false) {
    for (side=[-1,1]) {
        hull() one_side_key_placement(side)
            square([inter_switch_x, inter_switch_y], center=true);
        extreme_thumb_key_placement(side)
            square([inter_switch_x, inter_switch_y], center=true);
    }
    translate([0, top/2]) square([6*inter_switch_x, top], center=true);
    if (with_interior) {
        hull() {
            translate([0, 0.5]) square(center=true);
            for (side=[-1,1])
                extreme_thumb_key_placement(side)
                    square([inter_switch_x, inter_switch_y], center=true);
        }
    }
}

module sod323() {
    color([0.8,0.8,0.8]) linear_extrude(0.15) square([2.7,0.35], center=true);
    color([0.1,0.1,0.1]) linear_extrude(1.1) square([1.8,1.4], center=true);
}

module diode_placement() {
    key_placement() translate([0,4.7]) children();
}

module pcb() {
    translate([0,0,-pcb_depth]) {
        // FR4
        color([0,0.5,0]) {
            difference() {
                translate([0,0,-1.6]) linear_extrude(1.6) outline(true);
                key_placement() {
                    cylinder(d=3.429, h=4, center=true);
                    for (i=[-5.5,5.5]) translate([i,0]) cylinder(d=1.7018, h=4, center=true);
                    translate([0,-5.9]) cylinder(d=1.27, h=4, center=true);
                    translate([-5,-3.8]) cylinder(d=1.27, h=4, center=true);
                }
            }
        }
        // USB-C connector
        color([0.8,0.8,0.8]) translate([0,top,3.2/2]) rotate([90,0,0]) linear_extrude(7.5)
            rounded_square([9, 3.2], r=1, center=true);
        // MCU
        color([0.1,0.1,0.1]) translate([0,-17*2]) linear_extrude(1.6)
            rounded_square([10,10], r=1, center=true);
        // diodes
        diode_placement() sod323();
    }
}

module plate() {
    translate([0,0,-pcb_depth]) {
        difference() {
            linear_extrude(pcb_depth) difference() {
                outline(plate_with_interior);
                translate([0, top-7.5/2]) square([9+0.2, 7.5+0.2], center=true);
                key_placement() square([14, 14], center=true);
            }
            key_placement() linear_extrude(pcb_depth-1.2) square([15, 15], center=true);
        }
    }
}

module back() {
    color([0.2,0.2,0.2]) translate([0,0,-pcb_depth-1.6-0.6]) linear_extrude(0.6) outline(true);
}

color([0.5,0.5,0.5]) plate();
pcb();
back();
color([1,1,1,0.8]) key_placement() switch();
color([1,1,1]) key_placement() keycap();
