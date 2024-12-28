all: plate.svg plate.dxf fat_plate.stl fat_plate_visible_components.stl

%.stl: %.printing.scad utils.scad lib.scad
	openscad -o $@ $<
%.svg: %.2d.scad utils.scad lib.scad
	openscad -o $@ $<
%.dxf: %.2d.scad utils.scad lib.scad
	openscad -o $@ $<
