use std::fmt::Display;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[value(rename_all = "verbatim")]
pub enum FFlag {
    /// Sets level of detail culling for CSG models in studs. Type: integer. Accepted Values: 0-1000
    DFIntCSGLevelOfDetailSwitchingDistance,

    /// Sets level of detail culling for CSG models in studs (graphic levels 1 and 2). Type: integer. Accepted Values: 0-1000
    DFIntCSGLevelOfDetailSwitchingDistanceL12,

    /// Sets level of detail culling for CSG models in studs (graphic levels 2 and 3). Type: integer. Accepted Values: 0-1000
    DFIntCSGLevelOfDetailSwitchingDistanceL23,

    /// Sets level of detail culling for CSG models in studs (graphic levels 3 and 4). Type: integer. Accepted Values: 0-1000
    DFIntCSGLevelOfDetailSwitchingDistanceL34,
    /// Enables manual control for true fullscreen (is not relevant for Sober). Type: bool. Accepted Values: true/false
    FFlagHandleAltEnterFullscreenManually,

    /// Enables texture quality to be overrided by DFIntTextureQualityOverride. Type: bool. Accepted Values: true/false
    DFFlagTextureQualityOverrideEnabled,

    /// Sets texture quality level. (DFFlagTextureQualityOverrideEnabled must be set to true first). Type: integer. Accepted Values: 0-3
    DFIntTextureQualityOverride,

    /// Force MSAA anti-aliasing sample rate. Type: integer. Accepted Values: 1; 2; 4
    FIntDebugForceMSAASamples,

    /// Disables DPI downscaling (is not relevant for Sober). Type: bool. Accepted Values: true/false
    DFFlagDisableDPIScale,

    /// Prefers DirectX 11 for rendering (is not relevant for Sober). Type: bool. Accepted Values: true/false
    FFlagDebugGraphicsPreferD3D11,

    /// Overrides the skybox color to gray, removes atmospheric stars. Type: bool. Accepted Values: true/false
    FFlagDebugSkyGray,

    /// Disables voxel lighting. Type: bool. Accepted Values: true/false
    DFFlagDebugPauseVoxelizer,

    /// Overrides graphic quality level (does not affect render distance). Type: integer. Accepted Values: 0-21
    DFIntDebugFRMQualityLevelOverride,

    /// Sets the maximum distance for grass rendering in studs. Type: integer. Accepted Values: 0-1000
    FIntFRMMaxGrassDistance,

    /// Sets the minimum distance for grass rendering in studs. Type: integer. Accepted Values: 0-1000
    FIntFRMMinGrassDistance,

    /// Prefers Vulkan for rendering. Type: bool. Accepted Values: true/false
    FFlagDebugGraphicsPreferVulkan,

    /// Prefers OpenGL for rendering. Type: bool. Accepted Values: true/false
    FFlagDebugGraphicsPreferOpenGL,

    /// Reduces motion for grass. Type: bool. Accepted Values: true/false
    FIntGrassMovementReducedMotionFactor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FFlagType {
    Boolean(bool),
    Integer(i64),
    String(String),
}

impl Display for FFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for FFlagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FFlagType::Boolean(b) => write!(f, "{}", b),
            FFlagType::Integer(i) => write!(f, "{}", i),
            FFlagType::String(s) => write!(f, "{}", s),
        }
    }
}

impl FFlagType {
    pub fn parse(input: &str) -> Self {
        let trimmed = input.trim();

        if trimmed == "true" {
            return FFlagType::Boolean(true);
        }
        if trimmed == "false" {
            return FFlagType::Boolean(false);
        }

        if let Ok(int) = trimmed.parse::<i64>() {
            return FFlagType::Integer(int);
        }

        FFlagType::String(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bool() {
        assert_eq!(FFlagType::parse("true"), FFlagType::Boolean(true));
        assert_eq!(FFlagType::parse(" false "), FFlagType::Boolean(false));
    }

    #[test]
    fn parse_int() {
        assert_eq!(FFlagType::parse("123"), FFlagType::Integer(123));
        assert_eq!(FFlagType::parse("-1312"), FFlagType::Integer(-1312));
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            FFlagType::parse("meow"),
            FFlagType::String("meow".to_string())
        );
        assert_eq!(
            FFlagType::parse("12.3123"),
            FFlagType::String("12.3123".to_string())
        );
    }
}
