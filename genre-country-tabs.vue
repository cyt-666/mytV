
        <!-- 按类型浏览 -->
        <a-tab-pane key="genres" title="🎭 按类型">
          <div class="filter-toolbar">
            <a-space size="medium" wrap>
              <!-- 媒体类型切换 -->
              <a-radio-group v-model="genreMediaType" type="button" @change="handleGenreMediaTypeChange">
                <a-radio value="movies">电影</a-radio>
                <a-radio value="shows">剧集</a-radio>
              </a-radio-group>
              
              <!-- 数据源切换 -->
              <a-select 
                v-model="genreDataSource" 
                placeholder="数据源" 
                style="width: 160px;"
                @change="handleGenreDataSourceChange"
              >
                <a-option value="watched-weekly">📊 本周观看榜</a-option>
                <a-option value="watched-monthly">📈 本月观看榜</a-option>
                <a-option value="collected-monthly">⭐ 本月收藏榜</a-option>
              </a-select>
              
              <!-- 类型选择器 -->
              <a-select 
                v-model="selectedGenre" 
                placeholder="选择类型" 
                style="width: 180px;"
                @change="handleGenreChange"
              >
                <a-option value="all">🌟 全部类型</a-option>
                <a-option value="action">💥 动作</a-option>
                <a-option value="adventure">🗺️ 冒险</a-option>
                <a-option value="animation">🎨 动画</a-option>
                <a-option value="comedy">😄 喜剧</a-option>
                <a-option value="crime">🔫 犯罪</a-option>
                <a-option value="documentary">📹 纪录片</a-option>
                <a-option value="drama">🎭 剧情</a-option>
                <a-option value="family">👨‍👩‍👧 家庭</a-option>
                <a-option value="fantasy">🧙 奇幻</a-option>
                <a-option value="history">📜 历史</a-option>
                <a-option value="horror">👻 恐怖</a-option>
                <a-option value="music">🎵 音乐</a-option>
                <a-option value="mystery">🔍 悬疑</a-option>
                <a-option value="romance">💕 爱情</a-option>
                <a-option value="science-fiction">🚀 科幻</a-option>
                <a-option value="thriller">😱 惊悚</a-option>
                <a-option value="war">⚔️ 战争</a-option>
              </a-select>

              <!-- 结果计数 -->
              <a-tag color="arcoblue" v-if="genreFilteredItems.length > 0">
                <template #icon><icon-check-circle /></template>
                {{ genreFilteredItems.length }} 个结果
              </a-tag>
            </a-space>
          </div>

          <MediaGrid
            :items="genreFilteredItems"
            :loading="loading.genre && genreCurrentPage === 1"
            :loading-more="loadingMore.genre"
            :has-more="hasMoreGenre"
            @load-more="loadMoreGenreData"
            :media-type="genreMediaType === 'movies' ? 'movie' : 'show'"
            :empty-message="selectedGenre === 'all' ? '暂无数据' : `暂无${getGenreName(selectedGenre)}类型内容`"
          />
        </a-tab-pane>

        <!-- 按地区浏览 -->
        <a-tab-pane key="countries" title="🌍 按地区">
          <div class="filter-toolbar">
            <a-space size="medium" wrap>
              <!-- 媒体类型切换 -->
              <a-radio-group v-model="countryMediaType" type="button" @change="handleCountryMediaTypeChange">
                <a-radio value="movies">电影</a-radio>
                <a-radio value="shows">剧集</a-radio>
              </a-radio-group>
              
              <!-- 数据源切换 -->
              <a-select 
                v-model="countryDataSource" 
                placeholder="数据源" 
                style="width: 160px;"
                @change="handleCountryDataSourceChange"
              >
                <a-option value="watched-weekly">📊 本周观看榜</a-option>
                <a-option value="watched-monthly">📈 本月观看榜</a-option>
                <a-option value="collected-monthly">⭐ 本月收藏榜</a-option>
              </a-select>
              
              <!-- 地区选择器 -->
              <a-select 
                v-model="selectedCountry" 
                placeholder="选择地区" 
                style="width: 180px;"
                @change="handleCountryChange"
              >
                <a-option value="all">🌏 全部地区</a-option>
                <a-option value="us">🇺🇸 美国</a-option>
                <a-option value="gb">🇬🇧 英国</a-option>
                <a-option value="jp">🇯🇵 日本</a-option>
                <a-option value="kr">🇰🇷 韩国</a-option>
                <a-option value="cn">🇨🇳 中国</a-option>
                <a-option value="fr">🇫🇷 法国</a-option>
                <a-option value="de">🇩🇪 德国</a-option>
                <a-option value="ca">🇨🇦 加拿大</a-option>
                <a-option value="au">🇦🇺 澳大利亚</a-option>
                <a-option value="es">🇪🇸 西班牙</a-option>
                <a-option value="it">🇮🇹 意大利</a-option>
                <a-option value="in">🇮🇳 印度</a-option>
                <a-option value="hk">🇭🇰 香港</a-option>
                <a-option value="tw">🇹🇼 台湾</a-option>
              </a-select>

              <!-- 结果计数 -->
              <a-tag color="arcoblue" v-if="countryFilteredItems.length > 0">
                <template #icon><icon-check-circle /></template>
                {{ countryFilteredItems.length }} 个结果
              </a-tag>
            </a-space>
          </div>

          <MediaGrid
            :items="countryFilteredItems"
            :loading="loading.country && countryCurrentPage === 1"
            :loading-more="loadingMore.country"
            :has-more="hasMoreCountry"
            @load-more="loadMoreCountryData"
            :media-type="countryMediaType === 'movies' ? 'movie' : 'show'"
            :empty-message="selectedCountry === 'all' ? '暂无数据' : `暂无${getCountryName(selectedCountry)}地区内容`"
          />
        </a-tab-pane>
