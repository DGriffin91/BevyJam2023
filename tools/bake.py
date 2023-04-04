import bpy
import math
import bmesh

def set_vertex_colors(obj, color_layer_name, mat_index, values):
    mesh = obj.data
    bm = bmesh.new()
    bm.from_mesh(mesh)

    color_layer = bm.loops.layers.color.get(color_layer_name)

    if not color_layer:
        color_layer = bm.loops.layers.color.new(color_layer_name)

    for face in bm.faces:
        if face.material_index == mat_index:
            for loop in face.loops:
                loop[color_layer] = (values[0], values[1], loop[color_layer].z, loop[color_layer].w)

    bm.to_mesh(mesh)
    bm.free()

    # Set the color layer as the active render layer
    obj.data.vertex_colors[color_layer_name].active_render = True

def transfer_material_props_to_verts(obj):
  for idx, mat_slot in enumerate(obj.material_slots):
       if mat_slot.material and mat_slot.material.use_nodes:
           nodes = mat_slot.material.node_tree.nodes
           principled_bsdf = None
   
           for node in nodes:
               if node.type == 'BSDF_PRINCIPLED':
                   principled_bsdf = node
                   break
   
           if principled_bsdf:
               roughness_value = principled_bsdf.inputs['Roughness'].default_value
               metallic = principled_bsdf.inputs['Metallic'].default_value
               set_vertex_colors(obj, 'Attribute', idx, (roughness_value, metallic))

def proc(bake_res, resize_res, auto_smooth, unwrap):
    # Get selected objects
    selected_objects = bpy.context.selected_objects
    
    # Check if selected objects are mesh types and have at least one modifier
    mesh_objects = []
    for obj in selected_objects:
        if "boolean" in obj.name.lower():
            # Hide the object in the viewport
            obj.hide_viewport = True
            obj.hide_render = True
        if obj.type == 'CURVE':
            # Convert the curve to mesh
            bpy.ops.object.convert(target='MESH')
        if obj.type == 'MESH':
            mesh_objects.append(obj)
            bpy.ops.object.make_single_user(type='SELECTED_OBJECTS', object=True, obdata=True)
            transfer_material_props_to_verts(obj)
    
    if not mesh_objects:
        print("No suitable objects found!")
    else:
        # Apply all modifiers on selected objects
        for obj in mesh_objects:
            bpy.context.view_layer.objects.active = obj
            for modifier in obj.modifiers:
                bpy.ops.object.modifier_apply(modifier=modifier.name)

            if len(obj.data.materials) == 0:
                mat = bpy.data.materials.new(name="Material")
                mat.use_nodes = True
                obj.data.materials.append(mat)
                
            if obj.data.materials[0] is None:
                obj.data.materials[0] = bpy.data.materials.new(name="Material")
                obj.data.materials[0].use_nodes = True

        # Join selected objects into one mesh
        bpy.ops.object.join()
        if unwrap:
            # Smart UV project
            angle_limit = math.radians(66)
            island_margin = 0.003
            area_weight = 1.0
            correct_aspect = True
        
            bpy.ops.object.mode_set(mode='EDIT')
            bpy.ops.mesh.select_all(action='SELECT')
            bpy.ops.uv.smart_project(angle_limit=angle_limit, island_margin=island_margin, area_weight=area_weight, correct_aspect=correct_aspect)
            bpy.ops.object.mode_set(mode='OBJECT')
    
        # Create a new texture and assign it to every material
        texture = bpy.data.textures.new(name="Texture", type='IMAGE')
        image = bpy.data.images.new(name="Texture Image", width=bake_res, height=bake_res, float_buffer = True)
        for material in bpy.data.materials:
            # Check if material has a node tree
            if material.node_tree is not None:
                # Add a new texture node to the material's node tree
                nodes = material.node_tree.nodes
                texture_node = nodes.new(type='ShaderNodeTexImage')
                texture_node.image = image
                texture_node.select = True
                nodes.active = texture_node
                
        bpy.ops.object.select_all(action='DESELECT')
        bpy.context.active_object.select_set(True)
        obj.data.use_auto_smooth = auto_smooth
        
        # Run the bake
        bpy.ops.object.bake(type='COMBINED')
        
        image.scale(resize_res, resize_res)
    
        # Create a new material for the object
        material = bpy.data.materials.new(name="Material")
        material.use_nodes = True
        nodes = material.node_tree.nodes
        links = material.node_tree.links
    
        # Create an emission node and a texture node, and connect them to the output
        texture_node = nodes.new(type='ShaderNodeTexImage')
        texture_node.image = image
        emission_node = nodes.new(type='ShaderNodeEmission')
        links.new(texture_node.outputs[0], emission_node.inputs[0])
        links.new(emission_node.outputs[0], nodes["Material Output"].inputs[0])
        
        
        # Assign the material to the object
        obj = bpy.context.active_object
        obj.data.materials.clear()
        obj.data.materials.append(material)

        if len(obj.data.vertex_colors) > 0:
            obj.data.vertex_colors['Attribute'].active_render = True

bpy.context.scene.cycles.samples = 64
bpy.context.scene.cycles.adaptive_threshold = 0.1
proc(4096, 1024, auto_smooth = True, unwrap = True)

#for curtains
#proc(2048, 256, auto_smooth = False, unwrap = False)