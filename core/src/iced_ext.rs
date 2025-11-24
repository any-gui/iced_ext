use crate::{Border, Color, Gradient, Image, Rectangle, Shadow, Vector};
use crate::border::Radius;
use crate::gradient::Linear;
use crate::image::Handle;
use crate::renderer::{Quad, Style};
use std::ops::{Deref, Mul};
/// ExtPolygon
pub struct ExtPolygon {
    /// The bounds of the [`Quad`].
    pub bounds: Rectangle,

    /// The path of the Polygon. The coordinates need to be normalized to the range of 0-1.
    pub path: ExtPath,

    /// The [`Border`] of the [`Quad`]. The border is drawn on the inside of the [`Quad`].
    pub border: ExtBorder,

    /// The [`Shadow`] of the [`Quad`].
    pub shadow: ExtShadow,

    /// Whether the [`Quad`] should be snapped to the pixel grid.
    pub snap: bool,
}

impl ExtPolygon {
    /// Scale Alpha Of ExtPolygon
    pub fn scale_alpha(self, alpha: PercentF32) -> Self {
        let Self { bounds, path, border, shadow, snap } = self;
        Self {
            bounds,
            path,
            border: border.scale_alpha(alpha),
            shadow: shadow.scale_alpha(alpha),
            snap,
        }
    }
}

/// ExtPath
#[derive(Debug, Clone, PartialEq)]
pub enum ExtPath {
    /// Polygon path
    Polygon(Vec<Vector>),
    /// Quad path use bounds and Radius draw Quad
    Quad(Radius),
}

/// ExtBackground
#[derive(Debug, Clone, PartialEq)]
pub enum ExtBackground {
    /// A solid color.
    Color(Color),
    /// Linearly interpolate between several colors.
    Gradient(Gradient),
    /// Image variant Background
    Image(ExtImageBackground)
}

impl ExtBackground {
    /// Scale Alpha For ExtBackground
    pub fn scale_alpha(self, alpha: PercentF32) -> Self {
        match self {
            ExtBackground::Color(c) => ExtBackground::Color(c.scale_alpha(alpha)),
            ExtBackground::Gradient(g) => ExtBackground::Gradient(g.scale_alpha(*alpha)),
            ExtBackground::Image(i) => ExtBackground::Image(i.scale_alpha(alpha)),
        }
    }
}

impl From<Gradient> for ExtBackground {
    fn from(gradient: Gradient) -> Self {
        Self::Gradient(gradient)
    }
}

impl From<Linear> for ExtBackground {
    fn from(linear: Linear) -> Self {
        Self::Gradient(linear.into())
    }
}

/// 图片背景结构
#[derive(Debug, Clone, PartialEq)]
pub struct ExtImageBackground {
    /// 图片资源
    pub handle: Image<Handle>,
    /// 水平方向是否重复
    pub repeat_x: bool,
    /// 垂直方向是否重复
    pub repeat_y: bool,
    /// 位置，百分比 (0.0-1.0)
    pub position: (f32, f32),
    /// 图片尺寸策略
    pub size: ExtImageSize,
    /// Image Layer Alpha
    pub alpha: PercentF32,
}

impl ExtImageBackground {
    /// Scale Alpha for ExtImageBackground
    pub fn scale_alpha(mut self, alpha: PercentF32) -> Self {
        self.alpha = self.alpha * alpha;
        self
    }
}

/// 图片尺寸策略
#[derive(Debug, Clone, PartialEq)]
pub enum ExtImageSize {
    /// 原始大小
    Auto,
    /// 保持比例，覆盖整个容器
    Cover,
    /// 保持比例，完全显示
    Contain,
    /// 固定像素大小
    Px(u32, u32),
    /// 容器百分比
    Percent(f32, f32),
}

/// Dashed Line Cap
#[derive(Debug, Clone, PartialEq,Default)]
pub enum ExDashLineCap {
    /// None Cap
    #[default]
    None,
    /// Round Cap
    Round,
    /// Square Cap
    Square,
}

/// Dashed Stroke
#[derive(Debug, Clone, PartialEq,Default)]
pub struct ExDashedStroke {
    /// Length of the Dash in Pixel
    pub dash_length: f32,
    /// Dashed Gap
    pub gap: f32,
    /// Dashed Stroke
    pub cap: ExDashLineCap,
}
/// Ext Stroke Style
#[derive(Debug, Clone, PartialEq,Default)]
pub enum ExStrokeStyle {
    /// Solid Style
    #[default]
    Solid,
    /// Dashed Style
    Dashed(ExDashedStroke),
}

/// Line Join Type
#[derive(Debug, Clone, PartialEq,Default)]
pub enum ExLineJoin {
    /// Line Join Type Miter
    #[default]
    Miter,
    /// Line Join Type Round
    Round,
    /// Line Join Type Bevel
    Bevel
}
/// Stroke Used in border
#[derive(Debug, Clone, PartialEq,Default)]
pub struct  ExStroke {
    ///  Stroke Style
    style: ExStrokeStyle,
    /// Line Join Type
    join: ExLineJoin,
}

/// ExtBoarder
#[derive(Debug, Clone, PartialEq)]
pub struct ExtBorder {
    /// The Background of the border
    pub background: ExtBackground,
    /// Stroke Used By Boarder
    pub stroke: ExStroke,
    /// The width of the border.
    pub width: f32,
}

impl ExtBorder {
    /// ExtBoarder with given Radius
    pub fn rounded(radius: impl Into<Radius>) -> Self {
        Self {
            background: ExtBackground::Color(Color::TRANSPARENT),
            stroke: ExStroke::default(),
            width: 1.0,
        }
    }
    /// ExtBoarder with given Color
    pub fn from_color(color: impl Into<Color>) -> Self {
        Self {
            background: ExtBackground::Color(color.into()),
            stroke: ExStroke { style: ExStrokeStyle::Solid, join: ExLineJoin::Miter },
            width: 1.0,
        }
    }

    /// Scale Alpha For ExtBorder
    pub fn scale_alpha(mut self, scale: PercentF32) -> Self {
        self.background = self.background.scale_alpha(scale);
        self
    }
}

impl Default for ExtBorder {
    fn default() -> Self {
        Self {
            background: ExtBackground::Color(Color::TRANSPARENT),
            stroke: Default::default(),
            width: 0.0,
        }
    }
}

/// ExtBoxShadow
#[derive(Debug, Clone,Copy, Default, PartialEq)]
pub struct ExtBoxShadow {
    /// The color of the shadow.
    pub color: Color,

    /// The offset of the shadow.
    pub offset: Vector,

    /// The blur radius of the shadow.
    pub blur_radius: f32,

    /// The spread px of the shadow
    pub spread: f32,

    /// if this is a inner box shadow
    pub is_inset: bool,
}


impl ExtBoxShadow {
    /// Scale Alpha For Shadow
    pub fn scale_alpha(mut self, scale: PercentF32) -> Self {
        self.color = self.color.scale_alpha(scale);
        self
    }
}

/// ExtShadow
#[derive(Debug, Clone)]
pub struct ExtShadow {
    /// shadows
    pub shadows: Vec<ExtBoxShadow>,
}

impl ExtShadow {
    /// Scale Alpha For Shadow Vector
    pub fn scale_alpha(mut self, scale: PercentF32) -> Self {
        Self {
            shadows: self.shadows.into_iter().map(|s|s.scale_alpha(scale)).collect()
        }
    }
}

impl PartialEq for ExtShadow {
    fn eq(&self, other: &Self) -> bool {
        if self.shadows.len() != other.shadows.len() {
            false
        } else {
            let len = self.shadows.len();
            for i in 0..len {
                if self.shadows[i] != other.shadows[i] {
                    return false;
                }
            }
            true
        }
    }
}

impl Default for ExtShadow {
    fn default() -> Self {
        Self {
            shadows: vec![],
        }
    }
}

impl From<Color> for ExtBackground {
    fn from(color: Color) -> ExtBackground {
        ExtBackground::Color(color)
    }
}

/// Data type Used for Percent
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct PercentF32(f32);

impl PercentF32 {
    /// 创建，自动 clamp 到 [0, 1]
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// 原始值
    pub fn get(self) -> f32 {
        self.0
    }
}

impl From<f32> for PercentF32 {
    fn from(value: f32) -> Self {
        PercentF32::new(value)
    }
}

impl From<PercentF32> for f32 {
    fn from(p: PercentF32) -> f32 {
        p.0
    }
}

impl AsRef<f32> for PercentF32 {
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}

impl Deref for PercentF32 {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for PercentF32 {
    fn default() -> Self {
        Self(1.)
    }
}

// PercentF32 * f32 -> f32（最常见：缩放）
impl Mul<f32> for PercentF32 {
    type Output = f32;
    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}

// f32 * PercentF32 -> f32（可交换）
impl Mul<PercentF32> for f32 {
    type Output = f32;
    fn mul(self, rhs: PercentF32) -> Self::Output {
        self * rhs.0
    }
}

// PercentF32 * PercentF32 -> PercentF32（表示百分比叠乘）
impl Mul for PercentF32 {
    type Output = PercentF32;
    fn mul(self, rhs: PercentF32) -> Self::Output {
        PercentF32::new(self.0 * rhs.0)
    }
}