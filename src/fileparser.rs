use std::{ffi::OsStr, fs::{self, read, File}, os::raw, path::Path};
use image::{ImageFormat, ImageReader};
use unreal_asset::{asset::AssetTrait, cast, engine_version::EngineVersion, exports::{self, normal_export::NormalExport, Export, ExportBaseTrait}, properties::{array_property::ArrayProperty, color_property::ColorProperty, struct_property::StructProperty, Property}, types::{fname::FNameContainer, PackageIndex}, Asset};

fn rawrgb(path: &str) -> Result<Vec<u8>, String> {
    match fs::read(path) {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string()),
    }
}

fn image(path: &str) -> Result<Vec<u8>, String> {
    match image::ImageReader::open(path) {
        Ok(res) => {
            match res.decode() {
                Ok(res) => {
                    Ok(res.into_rgb8().into_raw())
                },
                Err(e) => Err(e.to_string()),
            }
        },
        Err(e) => Err(e.to_string()),
    }
}

//Shitty formats = Shitty code
fn uasset(path: &str) -> Result<Vec<u8>, String> {
    let Some(path) = path.strip_suffix(".uasset") else {
        return Err("File not .uasset".to_string())
    };
    // let mut data_file = File::open("ELA_BP_Title_in_00.uasset").unwrap();
    // let mut bulk_file = File::open("ELA_BP_Title_in_00.uexp").unwrap();
    let Ok(data_file) = File::open(format!("{}.uasset",path)) else {
        return Err("Failed to open .uasset".to_string())
    };
    let Ok(bulk_file) = File::open(format!("{}.uexp",path)) else {
        return Err("Failed to open .uexp".to_string())
    };
    let mut intermediate: Vec<Vec<[u8; 3]>> = Vec::new();
    let asset = Asset::new(data_file, Some(bulk_file), EngineVersion::VER_UE4_19).unwrap();
    let Some(x) = cast!(Export, NormalExport, &asset.asset_data.exports[0]) else {
        return Err("Error while parsing".to_string())
    };
    let Some(a) = cast!(Property, ArrayProperty, &x.properties[1]) else {
        return Err("Error while parsing".to_string())
    };
    for b in &a.value {
        let Some(c) = cast!(Property, StructProperty, &b) else {
            return Err("Error while parsing".to_string())
        };
        for d in &c.value {
            let Some(e) = cast!(Property, ArrayProperty, &d) else {
                return Err("Error while parsing".to_string())
            };
            let mut pixels: Vec<[u8; 3]> = Vec::new();
            for f in &e.value {
                let Some(g) = cast!(Property, StructProperty, &f) else {
                    return Err("Error while parsing".to_string())
                };
                for h in &g.value {
                    let Some(g) = cast!(Property, ColorProperty, &h) else {
                        return Err("Error while parsing".to_string())
                    };
                    pixels.push([g.color.g, g.color.b, g.color.a])
                }
            }
            intermediate.push(pixels)
        }
    }
    let mut output: Vec<u8> = Vec::new();
    for frame in intermediate {
        for y in 0..4 {
            for x in 0..60 {
                output.append(&mut frame[x*4+y].to_vec())
            }
        }
    }
    Ok(output)
}

pub fn parse(path: &str) -> Result<Vec<u8>, String> {
    match Path::new(path).extension().and_then(OsStr::to_str) {
        Some(s) => {
            if s == "uasset" {
                uasset(path)
            } else if ImageFormat::from_extension(s).is_some() {
                image(path)
            } else {
                rawrgb(path)
            }
        },
        None => rawrgb(path),
    }
}