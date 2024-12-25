all: plate.svg fat_plate.stl

%.stl: %.printing.scad
	openscad -o $@ $<
%.svg: %.2d.scad
	openscad -o $@ $<

plate.dxf: plate.2d.scad utils.scad lib.scad
fat_plate.stl: fat_plate.printing.scad utils.scad lib.scad
