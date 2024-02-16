/// A triangle, containing three indices, is the bedrock of a mesh.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Triangle {
    /// A triangle's vertices, notable for having two shared indices and a third unique one.
    indices: [u32; 3],
}

impl Triangle {
    /// Creates a new Triangle with three indices.
    ///
    /// # Safety
    /// You *must* ensure that triangles are created in counter-clockwise order.
    /// A square face will have the indices: 0 topleft, 1 topright, 2 bottomright, 3 bottomleft.
    pub fn new(a: u32, b: u32, c: u32) -> Self {
        Self { indices: [a, b, c] }
    }

    /// Offsets the values in self by `offset`. This is used
    /// to connect faces together.
    pub fn offset(&mut self, offset: u32) {
        self.indices = [self.a() + offset, self.b() + offset, self.c() + offset];
    }

    /// Gets the first index of the triangle.
    pub fn a(&self) -> u32 {
        self.indices[0]
    }

    /// Gets the second index of the triangle.
    pub fn b(&self) -> u32 {
        self.indices[1]
    }

    /// Gets the third index of the triangle.
    pub fn c(&self) -> u32 {
        self.indices[2]
    }

    /// 'Renders' a triangle into its constituent indices.
    pub fn render(self) -> [u32; 3] {
        self.indices
    }
}

/// One face (rectangle) of any 3D block.
///
/// It contains two triangles that make up its square.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Face {
    a: Triangle,
    b: Triangle,
}
impl Face {
    pub fn new(a: Triangle, b: Triangle, offset: u32) -> Self {
        let mut na = a;
        na.offset(offset);
        let mut nb = b;
        nb.offset(offset);

        Face { a: na, b: nb }
    }

    /// Offsets the verticies in the internal triangles by `offset`.
    pub fn offset(&mut self, offset: u32) {
        self.a.offset(offset);
        self.b.offset(offset);
    }

    /// Creates a `u32` list from self.
    pub fn to_u32_list(self) -> [u32; 6] {
        [
            self.a.a(),
            self.a.b(),
            self.a.c(),
            self.b.a(),
            self.b.b(),
            self.b.c(),
        ]
    }

    /// Returns the amount of internally-held indices.
    pub const fn count(&self) -> usize {
        6 // it'll always be six, as we have two triangles per face and three indices per triangle.
    }
}
