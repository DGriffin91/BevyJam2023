import bpy
import math
import bmesh
import os
import subprocess
import sys
import time

C = bpy.context


def export_gltf(filename):
    filename = filename.split()[0]
    filepath = os.path.join(bpy.path.abspath("//"), f"{filename}.gltf")
    bpy.ops.export_scene.gltf(
        filepath=filepath,
        check_existing=False,
        convert_lighting_mode="SPEC",
        gltf_export_id="",
        export_format="GLTF_SEPARATE",
        ui_tab="GENERAL",
        export_copyright="",
        export_image_format="AUTO",
        export_texture_dir="",
        export_jpeg_quality=75,
        export_keep_originals=False,
        export_texcoords=True,
        export_normals=True,
        export_tangents=False,
        export_materials="EXPORT",
        export_original_specular=False,
        export_colors=True,
        export_attributes=True,
        use_mesh_edges=False,
        use_mesh_vertices=False,
        export_cameras=False,
        use_selection=True,
        use_visible=False,
        use_renderable=False,
        use_active_collection_with_nested=True,
        use_active_collection=False,
        use_active_scene=False,
        export_extras=True,
        export_yup=True,
        export_apply=True,
        export_animations=True,
        export_frame_range=True,
        export_frame_step=1,
        export_force_sampling=True,
        export_nla_strips=True,
        export_nla_strips_merged_animation_name="Animation",
        export_def_bones=True,
        export_optimize_animation_size=False,
        export_anim_single_armature=True,
        export_reset_pose_bones=True,
        export_current_frame=False,
        export_skins=True,
        export_all_influences=False,
        export_morph=True,
        export_morph_normal=True,
        export_morph_tangent=False,
        export_lights=True,
        will_save_settings=True,
        filter_glob="*.gltf",
    )
    return filepath


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
                loop[color_layer] = (
                    values[0],
                    values[1],
                    loop[color_layer].z,
                    loop[color_layer].w,
                )

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
                if node.type == "BSDF_PRINCIPLED":
                    principled_bsdf = node
                    break

            if principled_bsdf:
                roughness_value = principled_bsdf.inputs["Roughness"].default_value
                metallic = principled_bsdf.inputs["Metallic"].default_value
                set_vertex_colors(obj, "Attribute", idx, (roughness_value, metallic))


def convert_exr_to_ktx2(input_file, output_file):
    cmd = [
        "CompressonatorCLI ",
        "-fd",
        "BC6H",
        "-mipsize",
        "256",
        input_file,
        output_file,
    ]

    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    if result.returncode != 0:
        print("Error: ", result.stderr.decode())
    else:
        print("Conversion successful: ", result.stdout.decode())


def save_image_as_exr(img):
    if img is None:
        print("Image not found!")
        return

    if img.file_format != "OPEN_EXR":
        img.file_format = "OPEN_EXR"

    image_settings = bpy.context.scene.render.image_settings
    image_settings.file_format = "OPEN_EXR"

    path = os.path.join(bpy.path.abspath("//"), f"{img.name}.exr")
    ktx_path = os.path.join(bpy.path.abspath("//"), f"{img.name}.ktx2")

    img.filepath_raw = path

    print(path)

    img.save_render(path)
    convert_exr_to_ktx2(path, ktx_path)


def auto_settings():
    args = {"bake_res": 4096, "resize_res": 1024, "auto_smooth": False, "unwrap": False}
    for i, word in enumerate(C.collection.name.split()[1:]):
        if i == 0 and word.isdigit():
            args["bake_res"] = int(word)
        elif i == 1 and word.isdigit():
            args["resize_res"] = int(word)
        elif word == "AS":
            args["auto_smooth"] = True
        elif word == "U":
            args["unwrap"] = True
    return args


def modify_gltf(filepath):
    with open(filepath, "r") as f:
        data = f.read()
        data = data.replace('"mimeType":"image/png",', "")
        data = data.replace('"mimeType":"image/jpg",', "")
        data = data.replace("png", "ktx2")
        file_root = bpy.path.basename(bpy.data.filepath).split(".")[0]
        print(f"{file_root}.ktx2", f"{get_name()}.ktx2")
        data = data.replace(f"{file_root}.ktx2", f"{get_name()}_lightmap.ktx2")

    with open(filepath, "w") as f:
        f.write(data)


def get_name():
    file_name = bpy.path.basename(bpy.data.filepath)
    words = C.collection.name.split()
    if len(words) > 0:
        return f"{file_name}_{words[0]}"
    else:
        return f"{file_name}_{C.collection.name}"


def delete_file_if_recent(filepath, time_threshold=30):
    if os.path.exists(filepath):
        current_time = time.time()
        file_mtime = os.path.getmtime(filepath)
        time_difference = current_time - file_mtime

        if time_difference <= time_threshold:
            os.remove(filepath)
            print(f"File '{filepath}' deleted.")
        else:
            print(
                f"File '{filepath}' was not saved within the last {time_threshold} seconds."
            )
    else:
        print(f"File '{filepath}' not found.")


def delete_intermediate():
    name = get_name()
    delete_file_if_recent(os.path.join(bpy.path.abspath("//"), f"{name}_lightmap.png"))
    # delete_file_if_recent(os.path.join(bpy.path.abspath("//"), f"{name}_lightmap.exr"))


def override(obj):
    return {
        "selected_objects": bpy.context.selected_objects,
        "active_object": bpy.context.view_layer.objects.active,
        "object": obj,
        "scene": bpy.context.scene,
        "area": next(
            area for area in bpy.context.screen.areas if area.type == "VIEW_3D"
        ),
        "region": next(
            region for region in bpy.context.area.regions if region.type == "WINDOW"
        ),
        "window": bpy.context.window,
        "screen": bpy.context.screen,
        "workspace": bpy.context.workspace,
    }


def disable_shadows():
    disabled_shadows = []
    selected_objects = bpy.context.selected_objects
    for obj in selected_objects:
        if obj.type == "LIGHT" and obj.data.use_shadow:
            obj.data.use_shadow = False
            disabled_shadows.append(obj)
    return disabled_shadows


def proc(bake_res, resize_res, auto_smooth, unwrap):
    # Get selected objects
    selected_objects = bpy.context.selected_objects

    # Check if selected objects are mesh types and have at least one modifier
    mesh_objects = []
    print(
        "hide bools, convert text/curve to mesh, transfer material props to vertex attributes"
    )
    for obj in selected_objects:
        if "boolean" in obj.name.lower():
            # Hide the object in the viewport
            obj.hide_viewport = True
            obj.hide_render = True
            obj.select_set(False)
        else:
            bpy.ops.object.make_single_user(
                type="SELECTED_OBJECTS", object=True, obdata=True
            )
            if obj.type == "CURVE" or obj.type == "FONT":
                # Convert the curve/font to mesh
                selection = obj.select_get()
                obj.select_set(True)
                bpy.context.view_layer.objects.active = obj
                bpy.ops.object.convert(override(obj), target="MESH")
                obj.select_set(selection)
            if obj.type == "MESH":
                mesh_objects.append(obj)
                transfer_material_props_to_verts(obj)

    if not mesh_objects:
        print("No suitable objects found!")
    else:
        # Apply all modifiers on selected objects
        print("apply all modifiers / init materials")
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
        print("Join")
        bpy.ops.object.join()
        if unwrap:
            print("Unwrap")
            # Smart UV project
            angle_limit = math.radians(66)
            island_margin = 0.001
            area_weight = 1.0
            correct_aspect = True

            bpy.ops.object.mode_set(mode="EDIT")
            bpy.ops.mesh.select_all(action="SELECT")
            bpy.ops.uv.smart_project(
                angle_limit=angle_limit,
                island_margin=island_margin,
                area_weight=area_weight,
                correct_aspect=correct_aspect,
            )
            bpy.ops.object.mode_set(mode="OBJECT")

        obj = bpy.context.active_object

        # Create a new texture and assign it to every material
        texture = bpy.data.textures.new(name="Texture", type="IMAGE")

        print("create image")
        image = bpy.data.images.new(
            name=f"{get_name()}_lightmap",
            width=bake_res,
            height=bake_res,
            float_buffer=True,
        )
        print("add image to material")
        for material in bpy.data.materials:
            # Check if material has a node tree
            if material.node_tree is not None:
                # Add a new texture node to the material's node tree
                nodes = material.node_tree.nodes
                texture_node = nodes.new(type="ShaderNodeTexImage")
                texture_node.image = image
                texture_node.select = True
                nodes.active = texture_node

        bpy.ops.object.select_all(action="DESELECT")
        bpy.context.active_object.select_set(True)
        obj.data.use_auto_smooth = auto_smooth

        print("transform_apply")
        bpy.ops.object.transform_apply(location=False, rotation=False, scale=True)

        # Run the bake
        print("bake")
        bpy.ops.object.bake(type="COMBINED")

        print("scale {resize_res}")
        image.scale(resize_res, resize_res)
        print("save exr")
        save_image_as_exr(image)

        print("assign material")
        # Create a new material for the object
        material = bpy.data.materials.new(name="Material")
        material.use_nodes = True
        nodes = material.node_tree.nodes
        links = material.node_tree.links

        # Create an emission node and a texture node, and connect them to the output
        texture_node = nodes.new(type="ShaderNodeTexImage")
        texture_node.image = image
        emission_node = nodes.new(type="ShaderNodeEmission")
        links.new(texture_node.outputs[0], emission_node.inputs[0])
        links.new(emission_node.outputs[0], nodes["Material Output"].inputs[0])

        # Assign the material to the object
        obj.data.materials.clear()
        obj.data.materials.append(material)

        if len(obj.data.vertex_colors) > 0:
            obj.data.vertex_colors["Attribute"].active_render = True
    return disabled_shadows


bpy.context.scene.cycles.samples = 64
bpy.context.scene.cycles.adaptive_threshold = 0.1
disabled_shadows = disable_shadows()
proc(**auto_settings())
modify_gltf(export_gltf(get_name()))
for obj in disabled_shadows:
    obj.data.use_shadow = True
# delete_intermediate()

# for curtains
# proc(2048, 256, auto_smooth = False, unwrap = False)

"""
Process:
copy file and rename with Exp
Select collection to be baked
right click, pick select objects
run script
----
curtain:
needs to be already unwrapped
after bake re-connect to emit on pbr material
alpha 0.5
object props: AlphaMode premultiplied
"""
