// Automatically generated rust module for 'Map_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CameraConfig {
    pub position: Option<PathEditor::MapVector3>,
    pub rotation: Option<PathEditor::MapVector4>,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl<'a> MessageRead<'a> for CameraConfig {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.position = Some(r.read_message::<PathEditor::MapVector3>(bytes)?),
                Ok(18) => msg.rotation = Some(r.read_message::<PathEditor::MapVector4>(bytes)?),
                Ok(29) => msg.fov = r.read_float(bytes)?,
                Ok(37) => msg.aspect_ratio = r.read_float(bytes)?,
                Ok(45) => msg.near = r.read_float(bytes)?,
                Ok(53) => msg.far = r.read_float(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CameraConfig {
    fn get_size(&self) -> usize {
        0
        + self.position.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.rotation.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.fov == 0f32 { 0 } else { 1 + 4 }
        + if self.aspect_ratio == 0f32 { 0 } else { 1 + 4 }
        + if self.near == 0f32 { 0 } else { 1 + 4 }
        + if self.far == 0f32 { 0 } else { 1 + 4 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.position { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.rotation { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.fov != 0f32 { w.write_with_tag(29, |w| w.write_float(*&self.fov))?; }
        if self.aspect_ratio != 0f32 { w.write_with_tag(37, |w| w.write_float(*&self.aspect_ratio))?; }
        if self.near != 0f32 { w.write_with_tag(45, |w| w.write_float(*&self.near))?; }
        if self.far != 0f32 { w.write_with_tag(53, |w| w.write_float(*&self.far))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LightConfig {
    pub position: Option<PathEditor::MapVector3>,
    pub rotation: Option<PathEditor::MapVector4>,
    pub color: Option<PathEditor::MapVector4>,
    pub shadow_bias: f32,
    pub shadow_normal_bias: f32,
}

impl<'a> MessageRead<'a> for LightConfig {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.position = Some(r.read_message::<PathEditor::MapVector3>(bytes)?),
                Ok(18) => msg.rotation = Some(r.read_message::<PathEditor::MapVector4>(bytes)?),
                Ok(26) => msg.color = Some(r.read_message::<PathEditor::MapVector4>(bytes)?),
                Ok(37) => msg.shadow_bias = r.read_float(bytes)?,
                Ok(45) => msg.shadow_normal_bias = r.read_float(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LightConfig {
    fn get_size(&self) -> usize {
        0
        + self.position.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.rotation.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.color.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.shadow_bias == 0f32 { 0 } else { 1 + 4 }
        + if self.shadow_normal_bias == 0f32 { 0 } else { 1 + 4 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.position { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.rotation { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.color { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.shadow_bias != 0f32 { w.write_with_tag(37, |w| w.write_float(*&self.shadow_bias))?; }
        if self.shadow_normal_bias != 0f32 { w.write_with_tag(45, |w| w.write_float(*&self.shadow_normal_bias))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MapConfig {
    pub camera: Option<PathEditor::CameraConfig>,
    pub light: Option<PathEditor::LightConfig>,
    pub wave_queues: Vec<PathEditor::WaveQueue>,
    pub paths: Vec<PathEditor::PathData>,
}

impl<'a> MessageRead<'a> for MapConfig {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.camera = Some(r.read_message::<PathEditor::CameraConfig>(bytes)?),
                Ok(18) => msg.light = Some(r.read_message::<PathEditor::LightConfig>(bytes)?),
                Ok(26) => msg.wave_queues.push(r.read_message::<PathEditor::WaveQueue>(bytes)?),
                Ok(34) => msg.paths.push(r.read_message::<PathEditor::PathData>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MapConfig {
    fn get_size(&self) -> usize {
        0
        + self.camera.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.light.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.wave_queues.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.paths.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.camera { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.light { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.wave_queues { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.paths { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MapVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<'a> MessageRead<'a> for MapVector3 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.x = r.read_float(bytes)?,
                Ok(21) => msg.y = r.read_float(bytes)?,
                Ok(29) => msg.z = r.read_float(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MapVector3 {
    fn get_size(&self) -> usize {
        0
        + if self.x == 0f32 { 0 } else { 1 + 4 }
        + if self.y == 0f32 { 0 } else { 1 + 4 }
        + if self.z == 0f32 { 0 } else { 1 + 4 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.x != 0f32 { w.write_with_tag(13, |w| w.write_float(*&self.x))?; }
        if self.y != 0f32 { w.write_with_tag(21, |w| w.write_float(*&self.y))?; }
        if self.z != 0f32 { w.write_with_tag(29, |w| w.write_float(*&self.z))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MapVector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl<'a> MessageRead<'a> for MapVector4 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.x = r.read_float(bytes)?,
                Ok(21) => msg.y = r.read_float(bytes)?,
                Ok(29) => msg.z = r.read_float(bytes)?,
                Ok(37) => msg.w = r.read_float(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MapVector4 {
    fn get_size(&self) -> usize {
        0
        + if self.x == 0f32 { 0 } else { 1 + 4 }
        + if self.y == 0f32 { 0 } else { 1 + 4 }
        + if self.z == 0f32 { 0 } else { 1 + 4 }
        + if self.w == 0f32 { 0 } else { 1 + 4 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.x != 0f32 { w.write_with_tag(13, |w| w.write_float(*&self.x))?; }
        if self.y != 0f32 { w.write_with_tag(21, |w| w.write_float(*&self.y))?; }
        if self.z != 0f32 { w.write_with_tag(29, |w| w.write_float(*&self.z))?; }
        if self.w != 0f32 { w.write_with_tag(37, |w| w.write_float(*&self.w))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PathData {
    pub points: Vec<PathEditor::PathWayPointData>,
}

impl<'a> MessageRead<'a> for PathData {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.points.push(r.read_message::<PathEditor::PathWayPointData>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PathData {
    fn get_size(&self) -> usize {
        0
        + self.points.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.points { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PathWayPointData {
    pub position: Option<PathEditor::MapVector3>,
    pub reach_range: f32,
}

impl<'a> MessageRead<'a> for PathWayPointData {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.position = Some(r.read_message::<PathEditor::MapVector3>(bytes)?),
                Ok(21) => msg.reach_range = r.read_float(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PathWayPointData {
    fn get_size(&self) -> usize {
        0
        + self.position.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.reach_range == 0f32 { 0 } else { 1 + 4 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.position { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.reach_range != 0f32 { w.write_with_tag(21, |w| w.write_float(*&self.reach_range))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Wave {
    pub wait_time: f32,
    pub unit: u64,
    pub spawn_cool_down: f32,
    pub duration: f32,
    pub per_spawn_count: i32,
    pub path_index: i32,
}

impl<'a> MessageRead<'a> for Wave {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.wait_time = r.read_float(bytes)?,
                Ok(16) => msg.unit = r.read_uint64(bytes)?,
                Ok(29) => msg.spawn_cool_down = r.read_float(bytes)?,
                Ok(37) => msg.duration = r.read_float(bytes)?,
                Ok(40) => msg.per_spawn_count = r.read_int32(bytes)?,
                Ok(48) => msg.path_index = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Wave {
    fn get_size(&self) -> usize {
        0
        + if self.wait_time == 0f32 { 0 } else { 1 + 4 }
        + if self.unit == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.unit) as u64) }
        + if self.spawn_cool_down == 0f32 { 0 } else { 1 + 4 }
        + if self.duration == 0f32 { 0 } else { 1 + 4 }
        + if self.per_spawn_count == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.per_spawn_count) as u64) }
        + if self.path_index == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.path_index) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.wait_time != 0f32 { w.write_with_tag(13, |w| w.write_float(*&self.wait_time))?; }
        if self.unit != 0u64 { w.write_with_tag(16, |w| w.write_uint64(*&self.unit))?; }
        if self.spawn_cool_down != 0f32 { w.write_with_tag(29, |w| w.write_float(*&self.spawn_cool_down))?; }
        if self.duration != 0f32 { w.write_with_tag(37, |w| w.write_float(*&self.duration))?; }
        if self.per_spawn_count != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.per_spawn_count))?; }
        if self.path_index != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.path_index))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct WaveQueue {
    pub wait_time: f32,
    pub waves: Vec<PathEditor::Wave>,
}

impl<'a> MessageRead<'a> for WaveQueue {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.wait_time = r.read_float(bytes)?,
                Ok(18) => msg.waves.push(r.read_message::<PathEditor::Wave>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for WaveQueue {
    fn get_size(&self) -> usize {
        0
        + if self.wait_time == 0f32 { 0 } else { 1 + 4 }
        + self.waves.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.wait_time != 0f32 { w.write_with_tag(13, |w| w.write_float(*&self.wait_time))?; }
        for s in &self.waves { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

