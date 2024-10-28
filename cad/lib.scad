$fs=0.5;
$fa=1;

nb_cols=5;
nb_rows=3;
nb_thumbs=3;
inter_switch_x=18;
inter_switch_y=17;
deltas=[0,2,6,2,-10];// column stagger
hand_angle=15;
top=12;

module rounded_square(size=1, center=false, r=1) {
  offset(r)
    offset(-r)
      square(size, center=center);
}

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
        extreme_thumb_key_placement(side)
            square([inter_switch_x, inter_switch_y], center=true);
    }
    translate([0, top/2]) square([6*inter_switch_x, top], center=true);
}

difference() {
    outline();
    key_placement() square([14, 14], center=true);
}
