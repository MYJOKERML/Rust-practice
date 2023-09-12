#![feature(impl_trait_in_assoc_type)]

pub struct S;

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
	async fn get_item(&self, _req: volo_gen::volo::example::GetItemRequest) -> ::core::result::Result<volo_gen::volo::example::GetItemResponse, ::volo_thrift::AnyhowError>{
					Ok(Default::default())
				}
}

