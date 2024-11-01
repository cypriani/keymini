module rounded(r) {
    offset(r) offset(-r) children();
}

module rounded_square(size=1, center=false, r=1) {
    rounded(r) square(size, center=center);
}
