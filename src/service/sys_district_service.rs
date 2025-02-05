use salvo::Request;

use crate::model::result::{HttpPage, PageDto, WebResult, WebResultPage};
use crate::model::sys_district::{SysDistrict, SysDistrictPageReq, SysDistrictVo};

pub struct SysDistrictService;
impl SysDistrictService{
    pub async fn get_sys_district_by_page(req: &mut Request)->HttpPage<SysDistrictVo>{
        let page_dto = req.parse_queries::<PageDto>().unwrap_or_else(|_| PageDto{ page: 1, page_size: 1000 });
        let item = req.parse_queries::<SysDistrictPageReq>()?;
        let page = SysDistrict::select_page(item,page_dto).await?;
        let page_vo = WebResultPage::<SysDistrictVo>::from(page);
        Ok(WebResult::success_page(page_vo))
    }
}