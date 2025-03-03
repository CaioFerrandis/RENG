use std::{ffi::c_void, path::Path};

use gl::{types::{GLint, GLsizei, GLvoid}, GenerateMipmap, UNSIGNED_BYTE};

pub fn make_tex(path: &str) -> u32{
    let mut texture = 0;

    unsafe{
        let img = image::open(&Path::new(path)).expect("Failed to load texture");
        let img_rgba = img.flipv().to_rgba8();
        let data = img_rgba.as_raw();

        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);  
        
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGBA as GLint,
                       img.width() as GLsizei,
                       img.height() as GLsizei,
                       0,
                       gl::RGBA,
                       gl::UNSIGNED_BYTE,
                       &data[0] as *const u8 as *const GLvoid
                       );
        
        GenerateMipmap(gl::TEXTURE_2D);
        texture
    }
}

pub fn make_tex_from_data(texture_data: &[u8], width: u32, height: u32) -> u32 {
    let mut texture = 0;

    unsafe {
        // Generate a texture object
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        // Set texture parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // Load texture data into OpenGL
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as GLint,
            width as GLsizei,
            height as GLsizei,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            texture_data.as_ptr() as *const GLvoid,
        );

        // Generate mipmaps
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    texture
}
