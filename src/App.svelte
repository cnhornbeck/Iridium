<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface ImportResult {
    success: boolean;
    message: string;
    processed_mods: string[];
    failed_mods: string[];
  }

  interface ExportResult {
    success: boolean;
    message: string;
    mod_list: string;
    timestamp: string;
  }

  interface UpgradeResult {
    success: boolean;
    message: string;
    output: string;
  }

  let activeTab: "import" | "export" = "import";
  let modInput = "";
  let isImporting = false;
  let isExporting = false;
  let isUpgrading = false;
  let importResult: ImportResult | null = null;
  let exportResult: ExportResult | null = null;
  let upgradeResult: UpgradeResult | null = null;
  let feriumAvailable: boolean | null = null;

  onMount(() => {
    checkFerium();
  });

  async function checkFerium() {
    try {
      feriumAvailable = await invoke<boolean>("check_ferium_available");
    } catch (error) {
      feriumAvailable = false;
    }
  }

  async function handleImport() {
    if (!modInput.trim()) return;

    isImporting = true;
    importResult = null;

    try {
      const modList = modInput
        .split("\n")
        .map((line) => line.trim())
        .filter((line) => line.length > 0);

      const result = await invoke<ImportResult>("import_mods", { modList });
      importResult = result;

      if (result.success) {
        modInput = "";
      }
    } catch (error) {
      importResult = {
        success: false,
        message: `Error: ${error}`,
        processed_mods: [],
        failed_mods: [],
      };
    } finally {
      isImporting = false;
    }
  }

  async function handleExport() {
    isExporting = true;
    exportResult = null;

    try {
      const result = await invoke<ExportResult>("export_mods");
      exportResult = result;
    } catch (error) {
      exportResult = {
        success: false,
        message: `Error: ${error}`,
        mod_list: "",
        timestamp: new Date().toLocaleTimeString(),
      };
    } finally {
      isExporting = false;
    }
  }

  async function handleUpgrade() {
    isUpgrading = true;
    upgradeResult = null;

    try {
      const result = await invoke<UpgradeResult>("upgrade_mods");
      upgradeResult = result;
    } catch (error) {
      upgradeResult = {
        success: false,
        message: `Error: ${error}`,
        output: "",
      };
    } finally {
      isUpgrading = false;
    }
  }
</script>

{#if feriumAvailable === false}
  <div
    class="min-h-screen bg-gradient-to-br from-gray-900 via-blue-900 to-purple-900 flex items-center justify-center p-4"
  >
    <div
      class="bg-red-50 border border-red-200 rounded-lg p-6 max-w-md w-full text-center"
    >
      <div class="mx-auto h-12 w-12 text-red-500 mb-4">❌</div>
      <h2 class="text-xl font-semibold text-red-800 mb-2">Ferium Not Found</h2>
      <p class="text-red-600 mb-4">
        Iridium requires Ferium to be installed and available in your PATH.
      </p>
      <p class="text-sm text-red-500">
        Please install Ferium from{" "}
        <a
          href="https://github.com/gorilla-devs/ferium"
          class="underline hover:text-red-700"
          target="_blank"
          rel="noopener noreferrer"
        >
          github.com/gorilla-devs/ferium
        </a>
      </p>
      <button
        on:click={checkFerium}
        class="mt-4 px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
      >
        Check Again
      </button>
    </div>
  </div>
{:else if feriumAvailable === null}
  <div
    class="min-h-screen bg-gradient-to-br from-gray-900 via-blue-900 to-purple-900 flex items-center justify-center"
  >
    <div class="text-center">
      <div class="mx-auto h-8 w-8 animate-spin text-blue-400 mb-4">⟳</div>
      <p class="text-blue-200">Checking Ferium availability...</p>
    </div>
  </div>
{:else}
  <div
    class="min-h-screen bg-gradient-to-br from-gray-900 via-blue-900 to-purple-900 p-4"
  >
    <div class="max-w-4xl mx-auto">
      <!-- Header -->
      <div class="text-center mb-8">
        <div class="flex items-center justify-center mb-4">
          <h1 class="text-4xl font-bold text-white">Iridium</h1>
        </div>
        <p class="text-blue-200">Modern Ferium Mod Manager</p>
      </div>

      <!-- Tab Navigation -->
      <div class="flex bg-gray-800/50 p-1 rounded-lg mb-6 backdrop-blur-sm">
        <button
          on:click={() => (activeTab = "import")}
          class="flex-1 flex items-center justify-center py-3 px-4 rounded-md transition-all {activeTab ===
          'import'
            ? 'bg-blue-600 text-white shadow-lg'
            : 'text-gray-300 hover:text-white hover:bg-gray-700/50'}"
        >
          <span class="mr-2">⬇️</span>
          Import Mods
        </button>
        <button
          on:click={() => (activeTab = "export")}
          class="flex-1 flex items-center justify-center py-3 px-4 rounded-md transition-all {activeTab ===
          'export'
            ? 'bg-purple-600 text-white shadow-lg'
            : 'text-gray-300 hover:text-white hover:bg-gray-700/50'}"
        >
          <span class="mr-2">⬆️</span>
          Export Mods
        </button>
      </div>

      <!-- Content -->
      <div class="bg-gray-800/30 backdrop-blur-sm rounded-xl p-6 shadow-2xl">
        {#if activeTab === "import"}
          <div class="space-y-6">
            <!-- Upgrade Section -->
            <div
              class="bg-gradient-to-r from-green-800/20 to-emerald-800/20 border border-green-700/30 rounded-lg p-4"
            >
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="text-lg font-semibold text-green-300 mb-1">
                    Ready to Install?
                  </h3>
                  <p class="text-green-200/80 text-sm">
                    Download and install all queued mods to your profile
                  </p>
                </div>
                <button
                  on:click={handleUpgrade}
                  disabled={isUpgrading}
                  class="bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-700 hover:to-emerald-700 disabled:from-gray-600 disabled:to-gray-600 disabled:cursor-not-allowed text-white font-bold py-3 px-6 rounded-lg transition-all transform hover:scale-105 disabled:transform-none shadow-lg flex items-center"
                >
                  {#if isUpgrading}
                    <span class="mr-2 animate-spin">⟳</span>
                    Installing...
                  {:else}
                    <span class="mr-2">🚀</span>
                    Upgrade & Install Mods
                  {/if}
                </button>
              </div>

              <!-- Upgrade Results -->
              {#if upgradeResult}
                <div
                  class="mt-4 rounded-lg p-4 {upgradeResult.success
                    ? 'bg-green-900/30 border border-green-700/50'
                    : 'bg-red-900/30 border border-red-700/50'}"
                >
                  <div class="flex items-center">
                    <span class="mr-2"
                      >{upgradeResult.success ? "✅" : "❌"}</span
                    >
                    <span
                      class="font-medium {upgradeResult.success
                        ? 'text-green-300'
                        : 'text-red-300'}"
                    >
                      {upgradeResult.message}
                    </span>
                  </div>
                </div>
              {/if}
            </div>

            <!-- Import Section -->
            <div>
              <label
                for="mod-input"
                class="block text-sm font-medium text-gray-300 mb-2"
              >
                Mod Names or IDs (one per line)
              </label>
              <textarea
                id="mod-input"
                bind:value={modInput}
                placeholder="Enter mod names or IDs here...&#10;jei&#10;waystones&#10;iron-chests"
                class="w-full h-40 bg-gray-700/50 border border-gray-600 rounded-lg px-4 py-3 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
                disabled={isImporting}
              ></textarea>
            </div>

            <button
              on:click={handleImport}
              disabled={isImporting || !modInput.trim()}
              class="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-semibold py-3 px-6 rounded-lg transition-colors flex items-center justify-center"
            >
              {#if isImporting}
                <span class="mr-2 animate-spin">⟳</span>
                Importing Mods...
              {:else}
                <span class="mr-2">⬇️</span>
                Import Mods
              {/if}
            </button>

            <!-- Import Results -->
            {#if importResult}
              <div
                class="rounded-lg p-4 {importResult.success
                  ? 'bg-green-900/30 border border-green-700/50'
                  : 'bg-yellow-900/30 border border-yellow-700/50'}"
              >
                <div class="flex items-center mb-3">
                  <span class="mr-2">{importResult.success ? "✅" : "⚠️"}</span>
                  <span
                    class="font-medium {importResult.success
                      ? 'text-green-300'
                      : 'text-yellow-300'}"
                  >
                    {importResult.message}
                  </span>
                </div>

                {#if importResult.processed_mods.length > 0}
                  <div class="mb-3">
                    <p class="text-green-300 text-sm font-medium mb-1">
                      Successfully imported:
                    </p>
                    <div class="bg-green-900/20 rounded p-2">
                      {#each importResult.processed_mods as mod}
                        <span
                          class="inline-block bg-green-700/30 text-green-200 text-xs px-2 py-1 rounded mr-1 mb-1"
                        >
                          {mod}
                        </span>
                      {/each}
                    </div>
                  </div>
                {/if}

                {#if importResult.failed_mods.length > 0}
                  <div>
                    <p class="text-red-300 text-sm font-medium mb-1">
                      Failed to import:
                    </p>
                    <div class="bg-red-900/20 rounded p-2">
                      {#each importResult.failed_mods as mod}
                        <span
                          class="inline-block bg-red-700/30 text-red-200 text-xs px-2 py-1 rounded mr-1 mb-1"
                        >
                          {mod}
                        </span>
                      {/each}
                    </div>
                  </div>
                {/if}
              </div>
            {/if}
          </div>
        {:else}
          <div class="space-y-6">
            <div class="text-center">
              <p class="text-gray-300 mb-4">
                Export your currently installed mods to clipboard
              </p>
              <button
                on:click={handleExport}
                disabled={isExporting}
                class="bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-semibold py-3 px-8 rounded-lg transition-colors flex items-center justify-center mx-auto"
              >
                {#if isExporting}
                  <span class="mr-2 animate-spin">⟳</span>
                  Exporting...
                {:else}
                  <span class="mr-2">⬆️</span>
                  Export Mod List
                {/if}
              </button>
            </div>

            <!-- Export Results -->
            {#if exportResult}
              <div
                class="rounded-lg p-4 {exportResult.success
                  ? 'bg-green-900/30 border border-green-700/50'
                  : 'bg-red-900/30 border border-red-700/50'}"
              >
                <div class="flex items-center mb-3">
                  <span class="mr-2">{exportResult.success ? "✅" : "❌"}</span>
                  <span
                    class="font-medium {exportResult.success
                      ? 'text-green-300'
                      : 'text-red-300'}"
                  >
                    {exportResult.message}
                  </span>
                </div>

                {#if exportResult.mod_list}
                  <div class="bg-gray-900/50 rounded p-3 mt-3">
                    <p class="text-gray-400 text-sm mb-2">Exported mod list:</p>
                    <pre
                      class="text-gray-300 text-sm whitespace-pre-wrap max-h-40 overflow-y-auto">{exportResult.mod_list}</pre>
                  </div>
                {/if}

                <p class="text-gray-400 text-xs mt-2">
                  Exported at {exportResult.timestamp}
                </p>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}
