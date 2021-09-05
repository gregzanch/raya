import bpy

bpy.data.scenes['Scene']['max_order']=50
bpy.data.scenes['Scene']['ray_count']=10000

bpy.data.objects['source']['node_type']='source'
bpy.data.objects['source']['active']=True

bpy.data.objects['receiver']['node_type']='receiver'
bpy.data.objects['receiver']['active']=True
bpy.data.objects['receiver']['radius']=0.5

bpy.data.objects['gym']['node_type']='reflector'
bpy.data.objects['gym']['active']=True

bpy.data.objects['panel']['node_type']='reflector'
bpy.data.objects['panel']['active']=True