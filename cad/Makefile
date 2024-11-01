all: plate.stl

%.stl: %.printing.scad
	openscad -o $@ $<

plate.stl: plate.printing.scad utils.scad lib.scad
