// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use common_catalog::catalog::CatalogManager;
use common_catalog::table_context::TableContext;
use common_datablocks::DataBlock;
use common_datavalues::DataField;
use common_datavalues::DataSchemaRefExt;
use common_datavalues::Series;
use common_datavalues::SeriesFrom;
use common_datavalues::ToDataType;
use common_datavalues::Vu8;
use common_exception::Result;
use common_meta_app::schema::TableIdent;
use common_meta_app::schema::TableInfo;
use common_meta_app::schema::TableMeta;

use super::table::AsyncOneBlockSystemTable;
use super::table::AsyncSystemTable;
use crate::storages::Table;

pub struct CatalogsTable {
    table_info: TableInfo,
}

#[async_trait::async_trait]
impl AsyncSystemTable for CatalogsTable {
    const NAME: &'static str = "system.catalogs";

    fn get_table_info(&self) -> &TableInfo {
        &self.table_info
    }

    async fn get_full_data(&self, _ctx: Arc<dyn TableContext>) -> Result<DataBlock> {
        let cm = CatalogManager::instance();

        let catalog_names: Vec<String> = cm.catalogs.iter().map(|x| x.key().clone()).collect();

        Ok(DataBlock::create(self.table_info.schema(), vec![
            Series::from_data(catalog_names),
        ]))
    }
}

impl CatalogsTable {
    pub fn create(table_id: u64) -> Arc<dyn Table> {
        let schema = DataSchemaRefExt::create(vec![DataField::new("name", Vu8::to_data_type())]);

        let table_info = TableInfo {
            desc: "'system'.'catalogs'".to_string(),
            name: "catalogs".to_string(),
            ident: TableIdent::new(table_id, 0),
            meta: TableMeta {
                schema,
                engine: "SystemCatalogs".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        AsyncOneBlockSystemTable::create(CatalogsTable { table_info })
    }
}
