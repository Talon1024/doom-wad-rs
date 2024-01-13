#[cfg(test)]
mod tests {
    use doom_wad::wad::*;
    use std::error::Error;
    use std::fs::File;
    use std::io::Read;
    use futures::executor;

    #[test]
    fn can_load_wad() -> Result<(), Box<dyn Error>> {
        let wad = executor::block_on(DoomWad::load("tests/data/3difytest.wad"))?;
        assert!(matches!(wad.wtype, DoomWadType::PWAD));
        assert_ne!(wad.lumps.len(), 0);
/* 
        let results = [Ok(1), Ok(3)];

        let result: Result<Vec<_>, &str> = results.iter().cloned().collect();

        assert_eq!(Ok(vec![1, 3]), result);
 */

        let lump_names: Result<Box<[LumpName]>, LumpNameConvertError> = [
            b"PLAYPAL\0",
            b"COLORMAP",
            b"P_START\0",
            b"BAAAAAAD",
            b"METALT2\0",
            b"4DOT\0\0\0\0",
            b"4DOTR\0\0\0",
            b"4DOTG\0\0\0",
            b"GOODGRIE",
            b"LUNPOEG\0",
            b"TRIMM\0\0\0",
            b"P_END\0\0\0",
            b"FF_START",
            b"METALTF2",
            b"METALT2\0",
            b"F_END\0\0\0",
            b"PNAMES\0\0",
            b"TEXTURE1",
            b"MAP01\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP02\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP03\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP04\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP05\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP06\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP07\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP08\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP09\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP10\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP11\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP12\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP13\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP14\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP15\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP16\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP17\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP18\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP19\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP20\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP21\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP22\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SEGS\0\0\0\0",
            b"SSECTORS",
            b"NODES\0\0\0",
            b"SECTORS\0",
            b"REJECT\0\0",
            b"BLOCKMAP",
            b"MAP23\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SECTORS\0",
            b"MAP24\0\0\0",
            b"THINGS\0\0",
            b"LINEDEFS",
            b"SIDEDEFS",
            b"VERTEXES",
            b"SECTORS\0",
        ].into_iter().map(LumpName::try_from).collect();
        let lump_names = lump_names?;

        assert_eq!(wad.lumps.len(), lump_names.len());
        let lumps_and_names = wad.lumps.iter().map(|l| l.name)
            .zip(lump_names.iter().copied());
        for lump_and_name in lumps_and_names {
            match lump_and_name {
                (lump, name) => {
                    assert_eq!(lump, name);
                }
            }
        }
        Ok(())
    }

    #[test]
    fn can_write_wad() -> Result<(), Box<dyn Error>> {
        let wad = executor::block_on(DoomWad::load("tests/data/3difytest.wad"))?;
        executor::block_on(wad.write("tests/data/another.wad"))?;

        let wad_file = File::open("tests/data/3difytest.wad")?;
        let out_wad_file = File::open("tests/data/another.wad")?;
        wad_file.bytes().zip(out_wad_file.bytes())
            .all(|bytes| bytes.0.unwrap() == bytes.1.unwrap());
        std::fs::remove_file("tests/data/another.wad")?;
        Ok(())
    }
}