use <utils.scad>

module keycap() {
    translate([0,0,4]) hull() {
        linear_extrude(0.1) square([17.5, 16.5], center=true);
        translate([0, 0, 3]) linear_extrude(0.1) rounded_square(12, r=1.5, center=true);
    }
}

module switch() {
    translate([0,0,-2.2]) linear_extrude(3) rounded_square([13.5,13.5], r=1, center=true);
    linear_extrude(3, scale=0.8) rounded_square([15, 15], r=1, center=true);
    translate([0,0,-2.2-2.65]) cylinder(h=5, d=4);
}
