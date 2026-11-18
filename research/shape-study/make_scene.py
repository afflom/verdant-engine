"""Create three meshes with the same outer extents and different topology."""
import numpy as np
import trimesh
from pathlib import Path
root=Path(__file__).parent
scene=trimesh.Scene()
ring=trimesh.creation.torus(major_radius=0.72,minor_radius=0.28,major_sections=88,minor_sections=28)
for mesh,name,x,color in [(trimesh.creation.box(extents=(2,2,2)),'Box',-2.7,[229,134,67,255]),(trimesh.creation.icosphere(subdivisions=4),'Sphere',0,[83,184,177,255]),(ring,'Ring',2.7,[193,137,216,255])]:
    mesh.vertices *= 2 / np.ptp(mesh.vertices,axis=0)
    mesh.visual.vertex_colors=color
    pose=np.eye(4); pose[:3,3]=[x,1.2,0]
    scene.add_geometry(mesh,node_name=name,geom_name=name,transform=pose)
(root/'shape-study.glb').write_bytes(scene.export(file_type='glb'))
