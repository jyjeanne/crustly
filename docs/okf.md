# Documentation

## Contents

- [Classes](#classes) (322)
- [Functions](#functions) (2264)
- [Interfaces](#interfaces) (7)
- [Modules](#modules) (125)
- [Packages](#packages) (1)

## Classes

### Message

*Rust Struct* — `benches/database.rs#L290-L295`

_private_

```
struct Message
```

### App

*Rust Struct* — `src/app/mod.rs#L12-L14`

```
pub struct App
```

### Cli

*Rust Struct* — `src/cli/mod.rs#L136-L156`

```
pub struct Cli
```

### Commands

*Rust Enum* — `src/cli/mod.rs#L159-L231`

```
pub enum Commands
```

### DbCommands

*Rust Enum* — `src/cli/mod.rs#L285-L296`

```
pub enum DbCommands
```

### KeyringCommands

*Rust Enum* — `src/cli/mod.rs#L299-L319`

```
pub enum KeyringCommands
```

### LogCommands

*Rust Enum* — `src/cli/mod.rs#L263-L282`

```
pub enum LogCommands
```

### OllamaCommands

*Rust Enum* — `src/cli/mod.rs#L234-L260`

```
pub enum OllamaCommands
```

### OutputFormat

*Rust Enum* — `src/cli/mod.rs#L322-L326`

```
pub enum OutputFormat
```

### Config

*Rust Struct* — `src/config/mod.rs#L202-L242`

```
pub struct Config
```

### DatabaseConfig

*Rust Struct* — `src/config/mod.rs#L583-L587`

```
pub struct DatabaseConfig
```

### DebugConfig

*Rust Struct* — `src/config/mod.rs#L246-L254`

```
pub struct DebugConfig
```

### LoggingConfig

*Rust Struct* — `src/config/mod.rs#L605-L613`

```
pub struct LoggingConfig
```

### McpConfig

*Rust Struct* — `src/config/mod.rs#L195-L198`

```
pub struct McpConfig
```

### McpServerConfig

*Rust Struct* — `src/config/mod.rs#L132-L137`

```
pub struct McpServerConfig
```

### MemoryConfig

*Rust Struct* — `src/config/mod.rs#L98-L108`

```
pub struct MemoryConfig
```

### OllamaModelConfig

*Rust Struct* — `src/config/mod.rs#L499-L533`

```
pub struct OllamaModelConfig
```

### OllamaProviderConfig

*Rust Struct* — `src/config/mod.rs#L445-L494`

```
pub struct OllamaProviderConfig
```

### PlanExecMode

*Rust Enum* — `src/config/mod.rs#L21-L29`

```
pub enum PlanExecMode
```

### PlanModeConfig

*Rust Struct* — `src/config/mod.rs#L33-L42`

```
pub struct PlanModeConfig
```

### ProviderConfig

*Rust Struct* — `src/config/mod.rs#L346-L362`

```
pub struct ProviderConfig
```

### ProviderConfigs

*Rust Struct* — `src/config/mod.rs#L258-L293`

```
pub struct ProviderConfigs
```

### QwenProviderConfig

*Rust Struct* — `src/config/mod.rs#L366-L417`

```
pub struct QwenProviderConfig
```

### SecurityConfig

*Rust Struct* — `src/config/mod.rs#L53-L63`

```
pub struct SecurityConfig
```

### ThinkSetting

*Rust Enum* — `src/config/mod.rs#L540-L543`

```
pub enum ThinkSetting
```

### ToolCacheConfig

*Rust Struct* — `src/config/mod.rs#L141-L154`

```
pub struct ToolCacheConfig
```

### CrabraceConfig

*Rust Struct* — `src/config/crabrace.rs#L10-L27`

```
pub struct CrabraceConfig
```

### CrabraceIntegration

*Rust Struct* — `src/config/crabrace.rs#L57-L60`

```
pub struct CrabraceIntegration
```

### ProviderSecrets

*Rust Struct* — `src/config/secrets.rs#L204-L209`

```
pub struct ProviderSecrets
```

### SecretString

*Rust Struct* — `src/config/secrets.rs#L21-L23`

```
pub struct SecretString
```

### ProviderUpdater

*Rust Struct* — `src/config/update.rs#L16-L19`

```
pub struct ProviderUpdater
```

### UpdateResult

*Rust Struct* — `src/config/update.rs#L193-L200`

```
pub struct UpdateResult
```

### Database

*Rust Struct* — `src/db/mod.rs#L21-L23`

```
pub struct Database
```

### Attachment

*Rust Struct* — `src/db/models.rs#L57-L66`

```
pub struct Attachment
```

### CompactionRecord

*Rust Struct* — `src/db/models.rs#L84-L93`

```
pub struct CompactionRecord
```

### File

*Rust Struct* — `src/db/models.rs#L46-L53`

```
pub struct File
```

### Message

*Rust Struct* — `src/db/models.rs#L28-L42`

```
pub struct Message
```

### Plan

*Rust Struct* — `src/db/models.rs#L97-L110`

```
pub struct Plan
```

### PlanTask

*Rust Struct* — `src/db/models.rs#L157-L174`

```
pub struct PlanTask
```

### PlanTaskStatus

*Rust Enum* — `src/db/models.rs#L114-L120`

```
pub enum PlanTaskStatus
```

### Session

*Rust Struct* — `src/db/models.rs#L12-L24`

```
pub struct Session
```

### ToolExecution

*Rust Struct* — `src/db/models.rs#L70-L80`

```
pub struct ToolExecution
```

### CompactionRecordRepository

*Rust Struct* — `src/db/repository/compaction.rs#L9-L11`

```
pub struct CompactionRecordRepository
```

### FileRepository

*Rust Struct* — `src/db/repository/file.rs#L13-L15`

```
pub struct FileRepository
```

### EpisodicMemoryRepository

*Rust Struct* — `src/db/repository/memory.rs#L9-L11`

```
pub struct EpisodicMemoryRepository
```

### MessageRepository

*Rust Struct* — `src/db/repository/message.rs#L12-L14`

```
pub struct MessageRepository
```

### PlanRepository

*Rust Struct* — `src/db/repository/plan.rs#L20-L22`

```
pub struct PlanRepository
```

### PlanTaskRepository

*Rust Struct* — `src/db/repository/plan.rs#L482-L484`

```
pub struct PlanTaskRepository
```

### SessionListOptions

*Rust Struct* — `src/db/repository/session.rs#L13-L20`

```
pub struct SessionListOptions
```

### SessionRepository

*Rust Struct* — `src/db/repository/session.rs#L24-L26`

```
pub struct SessionRepository
```

### DbRetryConfig

*Rust Struct* — `src/db/retry.rs#L18-L27`

```
pub struct DbRetryConfig
```

### CrustlyError

*Rust Enum* — `src/error.rs#L4-L30`

```
pub enum CrustlyError
```

### ErrorCode

*Rust Enum* — `src/error.rs#L33-L53`

```
pub enum ErrorCode
```

### CompactionRecord

*Rust Struct* — `src/llm/agent/compaction.rs#L10-L19`

```
pub struct CompactionRecord
```

### AgentContext

*Rust Struct* — `src/llm/agent/context.rs#L14-L35`

```
pub struct AgentContext
```

### TrackedFile

*Rust Struct* — `src/llm/agent/context.rs#L39-L44`

```
pub struct TrackedFile
```

### AgentError

*Rust Enum* — `src/llm/agent/error.rs#L8-L44`

```
pub enum AgentError
```

### CodebaseIndex

*Rust Struct* — `src/llm/agent/memory.rs#L51-L53`

```
pub struct CodebaseIndex
```

### CodebaseIndexEntry

*Rust Struct* — `src/llm/agent/memory.rs#L41-L48`

```
pub struct CodebaseIndexEntry
```

### EpisodicMemory

*Rust Struct* — `src/llm/agent/memory.rs#L14-L22`

```
pub struct EpisodicMemory
```

### SymbolKind

*Rust Enum* — `src/llm/agent/memory.rs#L29-L37`

```
pub enum SymbolKind
```

### AgentResponse

*Rust Struct* — `src/llm/agent/service.rs#L1740-L1769`

```
pub struct AgentResponse
```

### AgentService

*Rust Struct* — `src/llm/agent/service.rs#L262-L308`

```
pub struct AgentService
```

### AgentServiceLauncher

*Rust Struct* — `src/llm/agent/service.rs#L1791-L1797`

```
pub struct AgentServiceLauncher
```

### AgentStreamResponse

*Rust Struct* — `src/llm/agent/service.rs#L1772-L1784`

```
pub struct AgentStreamResponse
```

### FinalText

*Rust Struct* — `src/llm/agent/service.rs#L37-L44`

_private_

```
struct FinalText
```

### MockProvider

*Rust Struct* — `src/llm/agent/service.rs#L1886`

_private_

```
struct MockProvider
```

### MockProviderWithTools

*Rust Struct* — `src/llm/agent/service.rs#L2267-L2269`

_private_

```
struct MockProviderWithTools
```

### MockTool

*Rust Struct* — `src/llm/agent/service.rs#L2360`

_private_

```
struct MockTool
```

### ToolApprovalInfo

*Rust Struct* — `src/llm/agent/service.rs#L244-L253`

```
pub struct ToolApprovalInfo
```

### AnthropicError

*Rust Struct* — `src/llm/provider/anthropic.rs#L473-L475`

_private_

```
struct AnthropicError
```

### AnthropicErrorDetail

*Rust Struct* — `src/llm/provider/anthropic.rs#L478-L482`

_private_

```
struct AnthropicErrorDetail
```

### AnthropicProvider

*Rust Struct* — `src/llm/provider/anthropic.rs#L28-L31`

```
pub struct AnthropicProvider
```

### AnthropicRequest

*Rust Struct* — `src/llm/provider/anthropic.rs#L431-L448`

_private_

```
struct AnthropicRequest
```

### AnthropicResponse

*Rust Struct* — `src/llm/provider/anthropic.rs#L452-L458`

_private_

```
struct AnthropicResponse
```

### AnthropicTokenUsage

*Rust Struct* — `src/llm/provider/anthropic.rs#L462-L469`

_private_

```
struct AnthropicTokenUsage
```

### AzureOpenAIProvider

*Rust Struct* — `src/llm/provider/azure.rs#L13-L19`

```
pub struct AzureOpenAIProvider
```

### ProviderError

*Rust Enum* — `src/llm/provider/error.rs#L7-L63`

```
pub enum ProviderError
```

### FailoverProvider

*Rust Struct* — `src/llm/provider/factory.rs#L25-L27`

```
pub struct FailoverProvider
```

### RateLimitedProvider

*Rust Struct* — `src/llm/provider/factory.rs#L519`

_private_

```
struct RateLimitedProvider
```

### SucceedingProvider

*Rust Struct* — `src/llm/provider/factory.rs#L547-L549`

_private_

```
struct SucceedingProvider
```

### GeminiCandidate

*Rust Struct* — `src/llm/provider/gemini.rs#L860-L865`

_private_

```
struct GeminiCandidate
```

### GeminiContent

*Rust Struct* — `src/llm/provider/gemini.rs#L736-L740`

_private_

```
struct GeminiContent
```

### GeminiError

*Rust Struct* — `src/llm/provider/gemini.rs#L884-L888`

_private_

```
struct GeminiError
```

### GeminiErrorResponse

*Rust Struct* — `src/llm/provider/gemini.rs#L879-L881`

_private_

```
struct GeminiErrorResponse
```

### GeminiFunctionCall

*Rust Struct* — `src/llm/provider/gemini.rs#L784-L788`

_private_

```
struct GeminiFunctionCall
```

### GeminiFunctionCallingConfig

*Rust Struct* — `src/llm/provider/gemini.rs#L816-L818`

_private_

```
struct GeminiFunctionCallingConfig
```

### GeminiFunctionDeclaration

*Rust Struct* — `src/llm/provider/gemini.rs#L803-L807`

_private_

```
struct GeminiFunctionDeclaration
```

### GeminiFunctionResponse

*Rust Struct* — `src/llm/provider/gemini.rs#L791-L794`

_private_

```
struct GeminiFunctionResponse
```

### GeminiGenerationConfig

*Rust Struct* — `src/llm/provider/gemini.rs#L822-L839`

_private_

```
struct GeminiGenerationConfig
```

### GeminiInlineData

*Rust Struct* — `src/llm/provider/gemini.rs#L778-L781`

_private_

```
struct GeminiInlineData
```

### GeminiPart

*Rust Struct* — `src/llm/provider/gemini.rs#L744-L756`

_private_

```
struct GeminiPart
```

### GeminiProvider

*Rust Struct* — `src/llm/provider/gemini.rs#L38-L43`

```
pub struct GeminiProvider
```

### GeminiRequest

*Rust Struct* — `src/llm/provider/gemini.rs#L723-L733`

_private_

```
struct GeminiRequest
```

### GeminiResponse

*Rust Struct* — `src/llm/provider/gemini.rs#L851-L856`

_private_

```
struct GeminiResponse
```

### GeminiThinkingConfig

*Rust Struct* — `src/llm/provider/gemini.rs#L843-L847`

_private_

```
struct GeminiThinkingConfig
```

### GeminiTool

*Rust Struct* — `src/llm/provider/gemini.rs#L798-L800`

_private_

```
struct GeminiTool
```

### GeminiToolConfig

*Rust Struct* — `src/llm/provider/gemini.rs#L811-L813`

_private_

```
struct GeminiToolConfig
```

### GeminiUsageMetadata

*Rust Struct* — `src/llm/provider/gemini.rs#L869-L876`

_private_

```
struct GeminiUsageMetadata
```

### ModelOverrides

*Rust Struct* — `src/llm/provider/ollama.rs#L66-L77`

```
pub struct ModelOverrides
```

### OllamaProvider

*Rust Struct* — `src/llm/provider/ollama.rs#L131-L152`

```
pub struct OllamaProvider
```

### LocalModelInfo

*Rust Struct* — `src/llm/provider/ollama_models.rs#L20-L24`

```
pub struct LocalModelInfo
```

### ModelDetails

*Rust Struct* — `src/llm/provider/ollama_models.rs#L56-L61`

```
pub struct ModelDetails
```

### PullProgress

*Rust Struct* — `src/llm/provider/ollama_models.rs#L28-L38`

```
pub struct PullProgress
```

### AuthStyle

*Rust Enum* — `src/llm/provider/openai.rs#L53-L56`

_private_

```
enum AuthStyle
```

### OpenAIChoice

*Rust Struct* — `src/llm/provider/openai.rs#L1066-L1070`

_private_

```
struct OpenAIChoice
```

### OpenAIError

*Rust Struct* — `src/llm/provider/openai.rs#L1140-L1144`

_private_

```
struct OpenAIError
```

### OpenAIErrorResponse

*Rust Struct* — `src/llm/provider/openai.rs#L1135-L1137`

_private_

```
struct OpenAIErrorResponse
```

### OpenAIFunction

*Rust Struct* — `src/llm/provider/openai.rs#L1048-L1052`

_private_

```
struct OpenAIFunction
```

### OpenAIFunctionCall

*Rust Struct* — `src/llm/provider/openai.rs#L1036-L1039`

_private_

```
struct OpenAIFunctionCall
```

### OpenAIFunctionDelta

*Rust Struct* — `src/llm/provider/openai.rs#L1127-L1132`

_private_

```
struct OpenAIFunctionDelta
```

### OpenAIMessage

*Rust Struct* — `src/llm/provider/openai.rs#L1015-L1026`

_private_

```
struct OpenAIMessage
```

### OpenAIMessageDelta

*Rust Struct* — `src/llm/provider/openai.rs#L1101-L1108`

_private_

```
struct OpenAIMessageDelta
```

### OpenAIProvider

*Rust Struct* — `src/llm/provider/openai.rs#L60-L66`

```
pub struct OpenAIProvider
```

### OpenAIRequest

*Rust Struct* — `src/llm/provider/openai.rs#L964-L1005`

_private_

```
struct OpenAIRequest
```

### OpenAIResponse

*Rust Struct* — `src/llm/provider/openai.rs#L1055-L1062`

_private_

```
struct OpenAIResponse
```

### OpenAIStreamChoice

*Rust Struct* — `src/llm/provider/openai.rs#L1093-L1097`

_private_

```
struct OpenAIStreamChoice
```

### OpenAIStreamChunk

*Rust Struct* — `src/llm/provider/openai.rs#L1080-L1089`

_private_

```
struct OpenAIStreamChunk
```

### OpenAIStreamOptions

*Rust Struct* — `src/llm/provider/openai.rs#L1009-L1012`

_private_

```
struct OpenAIStreamOptions
```

### OpenAITool

*Rust Struct* — `src/llm/provider/openai.rs#L1042-L1045`

_private_

```
struct OpenAITool
```

### OpenAIToolCall

*Rust Struct* — `src/llm/provider/openai.rs#L1029-L1033`

_private_

```
struct OpenAIToolCall
```

### OpenAIToolCallDelta

*Rust Struct* — `src/llm/provider/openai.rs#L1113-L1123`

_private_

```
struct OpenAIToolCallDelta
```

### OpenAIUsage

*Rust Struct* — `src/llm/provider/openai.rs#L1073-L1076`

_private_

```
struct OpenAIUsage
```

### ToolCallBuilder

*Rust Struct* — `src/llm/provider/openai.rs#L654-L658`

_private_

```
struct ToolCallBuilder
```

### QwenChoice

*Rust Struct* — `src/llm/provider/qwen.rs#L1774-L1778`

_private_

```
struct QwenChoice
```

### QwenError

*Rust Struct* — `src/llm/provider/qwen.rs#L1843-L1847`

_private_

```
struct QwenError
```

### QwenErrorResponse

*Rust Struct* — `src/llm/provider/qwen.rs#L1838-L1840`

_private_

```
struct QwenErrorResponse
```

### QwenFunction

*Rust Struct* — `src/llm/provider/qwen.rs#L1758-L1762`

_private_

```
struct QwenFunction
```

### QwenFunctionCall

*Rust Struct* — `src/llm/provider/qwen.rs#L1746-L1749`

_private_

```
struct QwenFunctionCall
```

### QwenFunctionCallDelta

*Rust Struct* — `src/llm/provider/qwen.rs#L1830-L1835`

_private_

```
struct QwenFunctionCallDelta
```

### QwenMessage

*Rust Struct* — `src/llm/provider/qwen.rs#L1728-L1736`

_private_

```
struct QwenMessage
```

### QwenMessageDelta

*Rust Struct* — `src/llm/provider/qwen.rs#L1808-L1815`

_private_

```
struct QwenMessageDelta
```

### QwenProvider

*Rust Struct* — `src/llm/provider/qwen.rs#L111-L119`

```
pub struct QwenProvider
```

### QwenRequest

*Rust Struct* — `src/llm/provider/qwen.rs#L1704-L1725`

_private_

```
struct QwenRequest
```

### QwenResponse

*Rust Struct* — `src/llm/provider/qwen.rs#L1765-L1770`

_private_

```
struct QwenResponse
```

### QwenStreamChoice

*Rust Struct* — `src/llm/provider/qwen.rs#L1800-L1804`

_private_

```
struct QwenStreamChoice
```

### QwenStreamChunk

*Rust Struct* — `src/llm/provider/qwen.rs#L1788-L1796`

_private_

```
struct QwenStreamChunk
```

### QwenTool

*Rust Struct* — `src/llm/provider/qwen.rs#L1752-L1755`

_private_

```
struct QwenTool
```

### QwenToolCall

*Rust Struct* — `src/llm/provider/qwen.rs#L1739-L1743`

_private_

```
struct QwenToolCall
```

### QwenToolCallDelta

*Rust Struct* — `src/llm/provider/qwen.rs#L1820-L1827`

_private_

```
struct QwenToolCallDelta
```

### QwenUsage

*Rust Struct* — `src/llm/provider/qwen.rs#L1781-L1784`

_private_

```
struct QwenUsage
```

### SamplingOverrides

*Rust Struct* — `src/llm/provider/qwen.rs#L85-L89`

_private_

```
struct SamplingOverrides
```

### ThinkingConfig

*Rust Struct* — `src/llm/provider/qwen.rs#L75-L80`

```
pub struct ThinkingConfig
```

### ToolCallParser

*Rust Enum* — `src/llm/provider/qwen.rs#L54-L61`

```
pub enum ToolCallParser
```

### RetryConfig

*Rust Struct* — `src/llm/provider/retry.rs#L18-L29`

```
pub struct RetryConfig
```

### ModelRouter

*Rust Struct* — `src/llm/provider/router.rs#L19-L23`

```
pub struct ModelRouter
```

### ModelTier

*Rust Enum* — `src/llm/provider/router.rs#L8-L15`

```
pub enum ModelTier
```

### MockProvider

*Rust Struct* — `src/llm/provider/trait.rs#L93`

_private_

```
struct MockProvider
```

### ProviderCapabilities

*Rust Struct* — `src/llm/provider/trait.rs#L69-L74`

```
pub struct ProviderCapabilities
```

### CacheMetrics

*Rust Struct* — `src/llm/provider/types.rs#L272-L277`

```
pub struct CacheMetrics
```

### ContentBlock

*Rust Enum* — `src/llm/provider/types.rs#L58-L79`

```
pub enum ContentBlock
```

### ContentDelta

*Rust Enum* — `src/llm/provider/types.rs#L415-L422`

```
pub enum ContentDelta
```

### ImageSource

*Rust Enum* — `src/llm/provider/types.rs#L84-L89`

```
pub enum ImageSource
```

### LLMRequest

*Rust Struct* — `src/llm/provider/types.rs#L103-L153`

```
pub struct LLMRequest
```

### LLMResponse

*Rust Struct* — `src/llm/provider/types.rs#L293-L310`

```
pub struct LLMResponse
```

### Message

*Rust Struct* — `src/llm/provider/types.rs#L22-L27`

```
pub struct Message
```

### MessageDelta

*Rust Struct* — `src/llm/provider/types.rs#L426-L429`

```
pub struct MessageDelta
```

### PerfMetrics

*Rust Struct* — `src/llm/provider/types.rs#L317-L330`

```
pub struct PerfMetrics
```

### Role

*Rust Enum* — `src/llm/provider/types.rs#L11-L18`

```
pub enum Role
```

### StopReason

*Rust Enum* — `src/llm/provider/types.rs#L344-L353`

```
pub enum StopReason
```

### StreamEvent

*Rust Enum* — `src/llm/provider/types.rs#L374-L401`

```
pub enum StreamEvent
```

### StreamMessage

*Rust Struct* — `src/llm/provider/types.rs#L405-L410`

```
pub struct StreamMessage
```

### ThinkingConfig

*Rust Struct* — `src/llm/provider/types.rs#L94-L99`

```
pub struct ThinkingConfig
```

### TokenUsage

*Rust Struct* — `src/llm/provider/types.rs#L357-L362`

```
pub struct TokenUsage
```

### Tool

*Rust Struct* — `src/llm/provider/types.rs#L261-L268`

```
pub struct Tool
```

### AgentInput

*Rust Struct* — `src/llm/tools/agent.rs#L23-L32`

_private_

```
struct AgentInput
```

### AgentManifest

*Rust Struct* — `src/llm/tools/agent.rs#L35-L49`

_private_

```
struct AgentManifest
```

### AgentTool

*Rust Struct* — `src/llm/tools/agent.rs#L20`

```
pub struct AgentTool
```

### ApplyPatchInput

*Rust Struct* — `src/llm/tools/apply_patch.rs#L50-L53`

_private_

```
struct ApplyPatchInput
```

### ApplyPatchTool

*Rust Struct* — `src/llm/tools/apply_patch.rs#L47`

```
pub struct ApplyPatchTool
```

### FileOp

*Rust Enum* — `src/llm/tools/apply_patch.rs#L68-L81`

_private_

```
enum FileOp
```

### Hunk

*Rust Struct* — `src/llm/tools/apply_patch.rs#L63-L65`

_private_

```
struct Hunk
```

### HunkLine

*Rust Enum* — `src/llm/tools/apply_patch.rs#L56-L60`

_private_

```
enum HunkLine
```

### PlannedAction

*Rust Enum* — `src/llm/tools/apply_patch.rs#L289-L292`

_private_

```
enum PlannedAction
```

### AskUserInput

*Rust Struct* — `src/llm/tools/ask_user.rs#L18-L25`

_private_

```
struct AskUserInput
```

### AskUserTool

*Rust Struct* — `src/llm/tools/ask_user.rs#L15`

```
pub struct AskUserTool
```

### BashInput

*Rust Struct* — `src/llm/tools/bash.rs#L89-L120`

_private_

```
struct BashInput
```

### BashTool

*Rust Struct* — `src/llm/tools/bash.rs#L80`

```
pub struct BashTool
```

### CacheEntry

*Rust Struct* — `src/llm/tools/cache.rs#L28-L31`

_private_

```
struct CacheEntry
```

### CacheKey

*Rust Struct* — `src/llm/tools/cache.rs#L10-L13`

```
pub struct CacheKey
```

### ToolResultCache

*Rust Struct* — `src/llm/tools/cache.rs#L75-L78`

```
pub struct ToolResultCache
```

### ToolTtlConfig

*Rust Struct* — `src/llm/tools/cache.rs#L35-L44`

```
pub struct ToolTtlConfig
```

### CodeExecInput

*Rust Struct* — `src/llm/tools/code_exec.rs#L18-L32`

_private_

```
struct CodeExecInput
```

### CodeExecTool

*Rust Struct* — `src/llm/tools/code_exec.rs#L15`

```
pub struct CodeExecTool
```

### ContextEntry

*Rust Struct* — `src/llm/tools/context.rs#L19-L28`

_private_

```
struct ContextEntry
```

### ContextInput

*Rust Struct* — `src/llm/tools/context.rs#L124-L127`

_private_

```
struct ContextInput
```

### ContextOperation

*Rust Enum* — `src/llm/tools/context.rs#L84-L121`

_private_

```
enum ContextOperation
```

### ContextStore

*Rust Struct* — `src/llm/tools/context.rs#L31-L40`

_private_

```
struct ContextStore
```

### ContextTool

*Rust Struct* — `src/llm/tools/context.rs#L16`

```
pub struct ContextTool
```

### DocParserInput

*Rust Struct* — `src/llm/tools/doc_parser.rs#L21-L36`

_private_

```
struct DocParserInput
```

### DocParserTool

*Rust Struct* — `src/llm/tools/doc_parser.rs#L14`

```
pub struct DocParserTool
```

### DocumentMetadata

*Rust Struct* — `src/llm/tools/doc_parser.rs#L39-L48`

_private_

```
struct DocumentMetadata
```

### ParsedMetadata

*Rust Struct* — `src/llm/tools/doc_parser.rs#L223-L227`

_private_

```
struct ParsedMetadata
```

### EditInput

*Rust Struct* — `src/llm/tools/edit.rs#L63-L76`

_private_

```
struct EditInput
```

### EditOperation

*Rust Enum* — `src/llm/tools/edit.rs#L18-L60`

_private_

```
enum EditOperation
```

### EditTool

*Rust Struct* — `src/llm/tools/edit.rs#L14`

```
pub struct EditTool
```

### ToolError

*Rust Enum* — `src/llm/tools/error.rs#L7-L47`

```
pub enum ToolError
```

### FileFingerprint

*Rust Struct* — `src/llm/tools/file_read_cache.rs#L29-L32`

```
pub struct FileFingerprint
```

### FileReadCache

*Rust Struct* — `src/llm/tools/file_read_cache.rs#L61-L63`

```
pub struct FileReadCache
```

### ReadGate

*Rust Enum* — `src/llm/tools/file_read_cache.rs#L45-L52`

```
pub enum ReadGate
```

### GlobInput

*Rust Struct* — `src/llm/tools/glob.rs#L16-L31`

_private_

```
struct GlobInput
```

### GlobTool

*Rust Struct* — `src/llm/tools/glob.rs#L13`

```
pub struct GlobTool
```

### GrepInput

*Rust Struct* — `src/llm/tools/grep.rs#L17-L54`

_private_

```
struct GrepInput
```

### GrepTool

*Rust Struct* — `src/llm/tools/grep.rs#L14`

```
pub struct GrepTool
```

### HttpClientTool

*Rust Struct* — `src/llm/tools/http.rs#L15`

```
pub struct HttpClientTool
```

### HttpInput

*Rust Struct* — `src/llm/tools/http.rs#L18-L44`

_private_

```
struct HttpInput
```

### LsInput

*Rust Struct* — `src/llm/tools/ls.rs#L17-L33`

_private_

```
struct LsInput
```

### LsTool

*Rust Struct* — `src/llm/tools/ls.rs#L14`

```
pub struct LsTool
```

### Cell

*Rust Struct* — `src/llm/tools/notebook.rs#L78-L86`

_private_

```
struct Cell
```

### Notebook

*Rust Struct* — `src/llm/tools/notebook.rs#L70-L75`

_private_

```
struct Notebook
```

### NotebookEditTool

*Rust Struct* — `src/llm/tools/notebook.rs#L14`

```
pub struct NotebookEditTool
```

### NotebookInput

*Rust Struct* — `src/llm/tools/notebook.rs#L51-L62`

_private_

```
struct NotebookInput
```

### NotebookOperation

*Rust Enum* — `src/llm/tools/notebook.rs#L18-L48`

_private_

```
enum NotebookOperation
```

### PlanOperation

*Rust Enum* — `src/llm/tools/plan_tool.rs#L19-L115`

_private_

```
enum PlanOperation
```

### PlanTool

*Rust Struct* — `src/llm/tools/plan_tool.rs#L15`

```
pub struct PlanTool
```

### PowerShellInput

*Rust Struct* — `src/llm/tools/powershell.rs#L141-L160`

_private_

```
struct PowerShellInput
```

### PowerShellTool

*Rust Struct* — `src/llm/tools/powershell.rs#L138`

```
pub struct PowerShellTool
```

### ReadInput

*Rust Struct* — `src/llm/tools/read.rs#L26-L39`

_private_

```
struct ReadInput
```

### ReadTool

*Rust Struct* — `src/llm/tools/read.rs#L23`

```
pub struct ReadTool
```

### MockTool

*Rust Struct* — `src/llm/tools/registry.rs#L306-L309`

_private_

```
struct MockTool
```

### ToolRegistry

*Rust Struct* — `src/llm/tools/registry.rs#L35-L38`

```
pub struct ToolRegistry
```

### AllowAll

*Rust Struct* — `src/llm/tools/sandbox.rs#L392`

```
pub struct AllowAll
```

### AllowToolRule

*Rust Struct* — `src/llm/tools/sandbox.rs#L68-L70`

```
pub struct AllowToolRule
```

### AndPolicy

*Rust Struct* — `src/llm/tools/sandbox.rs#L333`

```
pub struct AndPolicy(pub Vec<Box<dyn PermissionPolicy>>)
```

### BashCommandAllowlist

*Rust Struct* — `src/llm/tools/sandbox.rs#L244-L246`

```
pub struct BashCommandAllowlist
```

### DenyPathPrefixRule

*Rust Struct* — `src/llm/tools/sandbox.rs#L91-L93`

```
pub struct DenyPathPrefixRule
```

### DenyToolRule

*Rust Struct* — `src/llm/tools/sandbox.rs#L45-L47`

```
pub struct DenyToolRule
```

### NotPolicy

*Rust Struct* — `src/llm/tools/sandbox.rs#L376`

```
pub struct NotPolicy(pub Box<dyn PermissionPolicy>)
```

### OrPolicy

*Rust Struct* — `src/llm/tools/sandbox.rs#L359`

```
pub struct OrPolicy(pub Vec<Box<dyn PermissionPolicy>>)
```

### PanicIfCalled

*Rust Struct* — `src/llm/tools/sandbox.rs#L615`

_private_

```
struct PanicIfCalled(Arc<AtomicBool>)
```

### PanicIfCalled

*Rust Struct* — `src/llm/tools/sandbox.rs#L641`

_private_

```
struct PanicIfCalled(Arc<AtomicBool>)
```

### PathBoundaryRule

*Rust Struct* — `src/llm/tools/sandbox.rs#L123-L125`

```
pub struct PathBoundaryRule
```

### PolicyDecision

*Rust Enum* — `src/llm/tools/sandbox.rs#L20-L26`

```
pub enum PolicyDecision
```

### SaveMemoryInput

*Rust Struct* — `src/llm/tools/save_memory.rs#L29-L31`

_private_

```
struct SaveMemoryInput
```

### SaveMemoryTool

*Rust Struct* — `src/llm/tools/save_memory.rs#L26`

```
pub struct SaveMemoryTool
```

### SkillInput

*Rust Struct* — `src/llm/tools/skill.rs#L24-L29`

_private_

```
struct SkillInput
```

### SkillListing

*Rust Struct* — `src/llm/tools/skill.rs#L213-L217`

```
pub struct SkillListing
```

### SkillOutput

*Rust Struct* — `src/llm/tools/skill.rs#L32-L38`

_private_

```
struct SkillOutput
```

### SkillTool

*Rust Struct* — `src/llm/tools/skill.rs#L21`

```
pub struct SkillTool
```

### SsrfSafeResolver

*Rust Struct* — `src/llm/tools/ssrf_guard.rs#L84`

```
pub struct SsrfSafeResolver
```

### FileLock

*Rust Struct* — `src/llm/tools/task.rs#L69-L71`

_private_

```
struct FileLock
```

### Task

*Rust Struct* — `src/llm/tools/task.rs#L40-L55`

_private_

```
struct Task
```

### TaskInput

*Rust Struct* — `src/llm/tools/task.rs#L268-L271`

_private_

```
struct TaskInput
```

### TaskOperation

*Rust Enum* — `src/llm/tools/task.rs#L222-L265`

_private_

```
enum TaskOperation
```

### TaskPriority

*Rust Enum* — `src/llm/tools/task.rs#L32-L37`

_private_

```
enum TaskPriority
```

### TaskStatus

*Rust Enum* — `src/llm/tools/task.rs#L22-L28`

_private_

```
enum TaskStatus
```

### TaskStore

*Rust Struct* — `src/llm/tools/task.rs#L58-L60`

_private_

```
struct TaskStore
```

### TaskTool

*Rust Struct* — `src/llm/tools/task.rs#L18`

```
pub struct TaskTool
```

### ReadInput

*Rust Struct* — `src/llm/tools/todo_write.rs#L95-L97`

_private_

```
struct ReadInput
```

### TodoInput

*Rust Enum* — `src/llm/tools/todo_write.rs#L121-L124`

_private_

```
enum TodoInput
```

### TodoItem

*Rust Struct* — `src/llm/tools/todo_write.rs#L60-L67`

```
pub struct TodoItem
```

### TodoItemInput

*Rust Struct* — `src/llm/tools/todo_write.rs#L106-L112`

_private_

```
struct TodoItemInput
```

### TodoPriority

*Rust Enum* — `src/llm/tools/todo_write.rs#L43-L47`

```
pub enum TodoPriority
```

### TodoStatus

*Rust Enum* — `src/llm/tools/todo_write.rs#L23-L28`

```
pub enum TodoStatus
```

### TodoStore

*Rust Struct* — `src/llm/tools/todo_write.rs#L70-L72`

_private_

```
struct TodoStore
```

### TodoWriteTool

*Rust Struct* — `src/llm/tools/todo_write.rs#L19`

```
pub struct TodoWriteTool
```

### WriteInput

*Rust Struct* — `src/llm/tools/todo_write.rs#L101-L103`

_private_

```
struct WriteInput
```

### ToolCapability

*Rust Enum* — `src/llm/tools/trait.rs#L166-L179`

```
pub enum ToolCapability
```

### ToolExecutionContext

*Rust Struct* — `src/llm/tools/trait.rs#L30-L64`

```
pub struct ToolExecutionContext
```

### ToolResult

*Rust Struct* — `src/llm/tools/trait.rs#L122-L134`

```
pub struct ToolResult
```

### WebFetchInput

*Rust Struct* — `src/llm/tools/web_fetch.rs#L27-L42`

_private_

```
struct WebFetchInput
```

### WebFetchTool

*Rust Struct* — `src/llm/tools/web_fetch.rs#L24`

```
pub struct WebFetchTool
```

### DuckDuckGoResponse

*Rust Struct* — `src/llm/tools/web_search.rs#L30-L46`

_private_

```
struct DuckDuckGoResponse
```

### RelatedTopic

*Rust Enum* — `src/llm/tools/web_search.rs#L50-L64`

_private_

```
enum RelatedTopic
```

### SearchInput

*Rust Struct* — `src/llm/tools/web_search.rs#L15-L22`

_private_

```
struct SearchInput
```

### TopicItem

*Rust Struct* — `src/llm/tools/web_search.rs#L67-L72`

_private_

```
struct TopicItem
```

### WebSearchTool

*Rust Struct* — `src/llm/tools/web_search.rs#L12`

```
pub struct WebSearchTool
```

### WriteInput

*Rust Struct* — `src/llm/tools/write.rs#L18-L30`

_private_

```
struct WriteInput
```

### WriteTool

*Rust Struct* — `src/llm/tools/write.rs#L15`

```
pub struct WriteTool
```

### LogConfig

*Rust Struct* — `src/logging.rs#L12-L30`

```
pub struct LogConfig
```

### LoggerGuard

*Rust Struct* — `src/logging.rs#L87-L90`

```
pub struct LoggerGuard
```

### McpServerStatus

*Rust Struct* — `src/mcp/mod.rs#L21-L27`

```
pub struct McpServerStatus
```

### JsonRpcRequest

*Rust Struct* — `src/mcp/client.rs#L13-L19`

_private_

```
struct JsonRpcRequest
```

### JsonRpcResponse

*Rust Struct* — `src/mcp/client.rs#L22-L29`

_private_

```
struct JsonRpcResponse
```

### MCPClient

*Rust Struct* — `src/mcp/client.rs#L74-L81`

```
pub struct MCPClient
```

### McpTool

*Rust Struct* — `src/mcp/client.rs#L292-L296`

```
pub struct McpTool
```

### McpToolDef

*Rust Struct* — `src/mcp/client.rs#L65-L69`

```
pub struct McpToolDef
```

### ResponseMatch

*Rust Enum* — `src/mcp/client.rs#L34-L42`

_private_

```
enum ResponseMatch
```

### AutoRunMode

*Rust Enum* — `src/plan/mod.rs#L802-L807`

```
pub enum AutoRunMode
```

### ExecutionSummary

*Rust Struct* — `src/plan/mod.rs#L417-L428`

```
pub struct ExecutionSummary
```

### InterruptedPlan

*Rust Struct* — `src/plan/mod.rs#L951-L956`

```
pub struct InterruptedPlan
```

### PauseReason

*Rust Enum* — `src/plan/mod.rs#L811-L816`

```
pub enum PauseReason
```

### PlanDocument

*Rust Struct* — `src/plan/mod.rs#L17-L56`

```
pub struct PlanDocument
```

### PlanModeState

*Rust Enum* — `src/plan/mod.rs#L822-L860`

```
pub enum PlanModeState
```

### PlanStatus

*Rust Enum* — `src/plan/mod.rs#L432-L447`

```
pub enum PlanStatus
```

### PlanTask

*Rust Struct* — `src/plan/mod.rs#L465-L518`

```
pub struct PlanTask
```

### TaskExecution

*Rust Struct* — `src/plan/mod.rs#L526-L544`

```
pub struct TaskExecution
```

### TaskStatus

*Rust Enum* — `src/plan/mod.rs#L756-L769`

```
pub enum TaskStatus
```

### TaskType

*Rust Enum* — `src/plan/mod.rs#L714-L735`

```
pub enum TaskType
```

### ToolCall

*Rust Struct* — `src/plan/mod.rs#L548-L563`

```
pub struct ToolCall
```

### ServiceContext

*Rust Struct* — `src/services/mod.rs#L21-L24`

```
pub struct ServiceContext
```

### ServiceManager

*Rust Struct* — `src/services/mod.rs#L41-L47`

```
pub struct ServiceManager
```

### FileService

*Rust Struct* — `src/services/file.rs#L14-L16`

```
pub struct FileService
```

### MessageService

*Rust Struct* — `src/services/message.rs#L13-L15`

```
pub struct MessageService
```

### PlanService

*Rust Struct* — `src/services/plan.rs#L43-L46`

```
pub struct PlanService
```

### PlanStatistics

*Rust Struct* — `src/services/plan.rs#L30-L39`

```
pub struct PlanStatistics
```

### PlanValidationWarning

*Rust Struct* — `src/services/plan.rs#L14-L18`

```
pub struct PlanValidationWarning
```

### WarningSeverity

*Rust Enum* — `src/services/plan.rs#L22-L26`

```
pub enum WarningSeverity
```

### SessionService

*Rust Struct* — `src/services/session.rs#L18-L20`

```
pub struct SessionService
```

### App

*Rust Struct* — `src/tui/app.rs#L67-L192`

```
pub struct App
```

### DisplayMessage

*Rust Struct* — `src/tui/app.rs#L19-L41`

```
pub struct DisplayMessage
```

### DummyProvider

*Rust Struct* — `src/tui/app.rs#L2532`

_private_

```
struct DummyProvider
```

### ErrorCategory

*Rust Enum* — `src/tui/error.rs#L55-L68`

```
pub enum ErrorCategory
```

### ErrorInfo

*Rust Struct* — `src/tui/error.rs#L86-L105`

```
pub struct ErrorInfo
```

### ErrorSeverity

*Rust Enum* — `src/tui/error.rs#L9-L18`

```
pub enum ErrorSeverity
```

### AppMode

*Rust Enum* — `src/tui/events.rs#L146-L179`

```
pub enum AppMode
```

### EventHandler

*Rust Struct* — `src/tui/events.rs#L182-L188`

```
pub struct EventHandler
```

### ToolApprovalRequest

*Rust Struct* — `src/tui/events.rs#L94-L115`

```
pub struct ToolApprovalRequest
```

### ToolApprovalResponse

*Rust Struct* — `src/tui/events.rs#L133-L142`

```
pub struct ToolApprovalResponse
```

### TuiEvent

*Rust Enum* — `src/tui/events.rs#L14-L90`

```
pub enum TuiEvent
```

### MarkdownRenderer

*Rust Struct* — `src/tui/markdown.rs#L31-L39`

_private_

```
struct MarkdownRenderer
```

### ModelPullProgress

*Rust Struct* — `src/tui/ollama_download.rs#L34-L38`

```
pub struct ModelPullProgress
```

### PromptAnalyzer

*Rust Struct* — `src/tui/prompt_analyzer.rs#L92-L100`

```
pub struct PromptAnalyzer
```

### DummyProvider

*Rust Struct* — `src/tui/render.rs#L2048`

_private_

```
struct DummyProvider
```

### DummyProvider

*Rust Struct* — `src/tui/runner.rs#L160`

_private_

```
struct DummyProvider
```

### RetryConfig

*Rust Struct* — `src/utils/retry.rs#L22-L33`

```
pub struct RetryConfig
```

### TestError

*Rust Struct* — `src/utils/retry.rs#L235-L238`

_private_

```
struct TestError
```

### ErrorMockProvider

*Rust Struct* — `tests/error_scenarios_test.rs#L24-L26`

_private_

```
struct ErrorMockProvider
```

### ErrorType

*Rust Enum* — `tests/error_scenarios_test.rs#L29-L35`

_private_

```
enum ErrorType
```

### WorkingMockProvider

*Rust Struct* — `tests/error_scenarios_test.rs#L318`

_private_

```
struct WorkingMockProvider
```

### MockProvider

*Rust Struct* — `tests/integration_test.rs#L24-L27`

_private_

```
struct MockProvider
```

### StreamingMockProvider

*Rust Struct* — `tests/streaming_test.rs#L18-L20`

_private_

```
struct StreamingMockProvider
```

## Functions

### bench_message_insert

*Rust Function* — `benches/database.rs#L170-L228`

_private_

```
fn bench_message_insert(c: &mut Criterion)
```

**Calls:** finish

### bench_message_query

*Rust Function* — `benches/database.rs#L231-L316`

_private_

```
fn bench_message_query(c: &mut Criterion)
```

**Calls:** finish

### bench_session_create

*Rust Function* — `benches/database.rs#L27-L61`

_private_

```
fn bench_session_create(c: &mut Criterion)
```

**Calls:** finish

### bench_session_get

*Rust Function* — `benches/database.rs#L64-L110`

_private_

```
fn bench_session_get(c: &mut Criterion)
```

**Calls:** finish

### bench_session_list

*Rust Function* — `benches/database.rs#L113-L167`

_private_

```
fn bench_session_list(c: &mut Criterion)
```

**Calls:** finish

### setup_test_db

*Rust Function* — `benches/database.rs#L16-L24`

_private_

```
async fn setup_test_db() -> (Database, TempDir)
```

**Calls:** run_migrations

### bench_parallel_dispatch

*Rust Function* — `benches/parallel_tool_dispatch.rs#L43-L61`

_private_

```
fn bench_parallel_dispatch(c: &mut Criterion)
```

**Calls:** make_temp_files, read_sequential, read_parallel, finish

### make_temp_files

*Rust Function* — `benches/parallel_tool_dispatch.rs#L12-L21`

_private_

```
fn make_temp_files(dir: &TempDir, n: usize) -> Vec<std::path::PathBuf>
```

**Called by:** bench_parallel_dispatch

### read_parallel

*Rust Function* — `benches/parallel_tool_dispatch.rs#L34-L41`

_private_

```
async fn read_parallel(paths: &[std::path::PathBuf]) -> Vec<String>
```

**Called by:** bench_parallel_dispatch

### read_sequential

*Rust Function* — `benches/parallel_tool_dispatch.rs#L24-L31`

_private_

```
async fn read_sequential(paths: &[std::path::PathBuf]) -> Vec<String>
```

**Calls:** len

**Called by:** bench_parallel_dispatch

### default

*Rust Method* — `src/app/mod.rs#L28-L30`

_private_

```
fn default() -> Self
```

### new

*Rust Method* — `src/app/mod.rs#L17-L19`

```
pub fn new() -> Result<Self>
```

### run

*Rust Method* — `src/app/mod.rs#L21-L24`

```
pub async fn run(&mut self) -> Result<()>
```

### is_rust_file_in_root

*Rust Function* — `src/app/mod.rs#L104-L106`

```
pub fn is_rust_file_in_root(path: &Path, root: &Path) -> bool
```

### start_file_watcher

*Rust Function* — `src/app/mod.rs#L38-L101`

```
pub fn start_file_watcher(pool: SqlitePool, project_root: PathBuf)
```

**Calls:** index_file

### auto_mode_bypasses_approval

*Rust Function* — `src/cli/mod.rs#L863-L872`

_private_

```
fn auto_mode_bypasses_approval(mode: &crate::config::PlanExecMode, tool_name: &str) -> bool
```

**Calls:** is_high_risk_tool

**Called by:** build_approval_callback

### auto_mode_bypasses_approval_autoplan_gates_high_risk_tools_only

*Rust Function* — `src/cli/mod.rs#L1442-L1468`

_private_

```
fn auto_mode_bypasses_approval_autoplan_gates_high_risk_tools_only()
```

### auto_mode_bypasses_approval_fullauto_bypasses_everything

*Rust Function* — `src/cli/mod.rs#L1491-L1508`

_private_

```
fn auto_mode_bypasses_approval_fullauto_bypasses_everything()
```

### auto_mode_bypasses_approval_interactive_never_bypasses

*Rust Function* — `src/cli/mod.rs#L1423-L1439`

_private_

```
fn auto_mode_bypasses_approval_interactive_never_bypasses()
```

### build_approval_callback

*Rust Function* — `src/cli/mod.rs#L696-L751`

_private_

```
fn build_approval_callback( event_sender: tokio::sync::mpsc::UnboundedSender<crate::tui::events::TuiEvent>, auto_mode: Arc<std::sync::Mutex<crate::config::PlanExecMode>>, ) -> crate::llm::agent::ApprovalCallback
```

**Calls:** auto_mode_bypasses_approval

**Called by:** cmd_chat

### build_tool_registry

*Rust Function* — `src/cli/mod.rs#L597-L638`

_private_

```
fn build_tool_registry() -> crate::llm::tools::registry::ToolRegistry
```

**Calls:** register

**Called by:** cmd_chat, cmd_run, build_tool_registry_registers_every_built_in_tool, connect_configured_mcp_servers_returns_empty_status_with_no_servers, connect_configured_mcp_servers_records_failure_for_unreachable_server

### build_tool_registry_registers_every_built_in_tool

*Rust Function* — `src/cli/mod.rs#L1522-L1554`

_private_

```
fn build_tool_registry_registers_every_built_in_tool()
```

**Calls:** build_tool_registry

### cmd_autoplan

*Rust Function* — `src/cli/mod.rs#L1205-L1228`

_private_

```
async fn cmd_autoplan( config: &crate::config::Config, goal: String, max_iterations: u32, ) -> Result<()>
```

**Calls:** cmd_run

**Called by:** run

### cmd_chat

*Rust Function* — `src/cli/mod.rs#L754-L844`

_private_

```
async fn cmd_chat(config: &crate::config::Config, _session_id: Option<String>) -> Result<()>
```

**Calls:** run_migrations, create_provider, build_tool_registry, connect_configured_mcp_servers, with_max_tool_iterations, set_ollama_host, ollama_host, set_ollama_config, set_mcp_status, event_sender, set_auto_mode_state, build_approval_callback, set_policy, to_policy, with_tool_registry, with_approval_callback, set_agent_service

**Called by:** run

### cmd_config

*Rust Function* — `src/cli/mod.rs#L437-L487`

_private_

```
async fn cmd_config(config: &crate::config::Config, show_secrets: bool) -> Result<()>
```

**Called by:** run

### cmd_db

*Rust Function* — `src/cli/mod.rs#L490-L591`

_private_

```
async fn cmd_db(config: &crate::config::Config, operation: DbCommands) -> Result<()>
```

**Calls:** run_migrations

**Called by:** run

### cmd_init

*Rust Function* — `src/cli/mod.rs#L405-L434`

_private_

```
async fn cmd_init(_config: &crate::config::Config, force: bool) -> Result<()>
```

**Called by:** run

### cmd_keyring

*Rust Function* — `src/cli/mod.rs#L993-L1088`

_private_

```
async fn cmd_keyring(operation: KeyringCommands) -> Result<()>
```

**Calls:** from_str, with_context, from_keyring_optional

**Called by:** run

### cmd_logs

*Rust Function* — `src/cli/mod.rs#L1231-L1393`

_private_

```
async fn cmd_logs(operation: LogCommands) -> Result<()>
```

**Calls:** len, get_log_path, is_empty, cleanup_old_logs

**Called by:** run

### cmd_ollama

*Rust Function* — `src/cli/mod.rs#L1103-L1194`

_private_

```
async fn cmd_ollama(config: &crate::config::Config, operation: OllamaCommands) -> Result<()>
```

**Calls:** ollama_host, list_models, is_empty, pull_model, with_context, delete_model, show_model, generate_embeddings, next

### cmd_ollama

*Rust Function* — `src/cli/mod.rs#L1197-L1202`

_private_

```
async fn cmd_ollama(_config: &crate::config::Config, _operation: OllamaCommands) -> Result<()>
```

### cmd_run

*Rust Function* — `src/cli/mod.rs#L875-L990`

_private_

```
async fn cmd_run( config: &crate::config::Config, prompt: String, auto_approve: bool, format: OutputFormat, ) -> Result<()>
```

**Calls:** run_migrations, create_provider, build_tool_registry, register_mcp_server, set_policy, to_policy, with_tool_registry, with_max_tool_iterations, with_auto_approve_tools, send_message_with_tools

**Called by:** run, cmd_autoplan

### connect_configured_mcp_servers

*Rust Function* — `src/cli/mod.rs#L646-L684`

_private_

```
async fn connect_configured_mcp_servers( tool_registry: &mut crate::llm::tools::registry::ToolRegistry, config: &crate::config::Config, ) -> Vec<crate::mcp::McpServerStatus>
```

**Calls:** register_mcp_server

**Called by:** cmd_chat, connect_configured_mcp_servers_returns_empty_status_with_no_servers, connect_configured_mcp_servers_records_failure_for_unreachable_server

### connect_configured_mcp_servers_records_failure_for_unreachable_server

*Rust Function* — `src/cli/mod.rs#L1567-L1582`

_private_

```
async fn connect_configured_mcp_servers_records_failure_for_unreachable_server()
```

**Calls:** build_tool_registry, connect_configured_mcp_servers

### connect_configured_mcp_servers_returns_empty_status_with_no_servers

*Rust Function* — `src/cli/mod.rs#L1557-L1564`

_private_

```
async fn connect_configured_mcp_servers_returns_empty_status_with_no_servers()
```

**Calls:** build_tool_registry, connect_configured_mcp_servers

### known_gap_powershell_is_not_classified_as_high_risk

*Rust Function* — `src/cli/mod.rs#L1471-L1488`

_private_

```
fn known_gap_powershell_is_not_classified_as_high_risk()
```

### load_config

*Rust Function* — `src/cli/mod.rs#L387-L402`

_private_

```
async fn load_config(config_path: Option<&str>) -> Result<crate::config::Config>
```

**Calls:** load_from_path, validate

**Called by:** run

### ollama_host

*Rust Function* — `src/cli/mod.rs#L1093-L1100`

_private_

```
fn ollama_host(config: &crate::config::Config) -> String
```

**Called by:** cmd_chat, cmd_ollama

### run

*Rust Function* — `src/cli/mod.rs#L329-L384`

```
pub async fn run() -> Result<()>
```

**Calls:** parse, load_config, override_default_model, cmd_chat, cmd_init, cmd_config, cmd_db, cmd_logs, cmd_keyring, cmd_run, cmd_autoplan

### test_cli_parse

*Rust Function* — `src/cli/mod.rs#L1400-L1403`

_private_

```
fn test_cli_parse()
```

### test_ollama_command_parses

*Rust Function* — `src/cli/mod.rs#L1511-L1519`

_private_

```
fn test_ollama_command_parses()
```

### test_ollama_host_defaults_when_unconfigured

*Rust Function* — `src/cli/mod.rs#L1406-L1409`

_private_

```
fn test_ollama_host_defaults_when_unconfigured()
```

### test_ollama_host_uses_configured_value

*Rust Function* — `src/cli/mod.rs#L1412-L1420`

_private_

```
fn test_ollama_host_uses_configured_value()
```

### apply_env_overrides

*Rust Method* — `src/config/mod.rs#L771-L813`

_private_

```
fn apply_env_overrides(mut config: Self) -> Result<Self>
```

**Calls:** parse, load_provider_api_keys

**Called by:** load, load_from_path, test_config_env_overrides, test_provider_config_api_keys_from_env, test_ollama_config_from_env

### default

*Rust Method* — `src/config/mod.rs#L629-L647`

_private_

```
fn default() -> Self
```

**Calls:** default_db_path, default_log_level

### load

*Rust Method* — `src/config/mod.rs#L658-L690`

```
pub fn load() -> Result<Self>
```

**Calls:** system_config_path, merge_from_file, project_config_path, local_config_path, apply_env_overrides

### load_from_path

*Rust Method* — `src/config/mod.rs#L698-L717`

```
pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self>
```

**Calls:** merge_from_file, apply_env_overrides

**Called by:** load_config

### load_provider_api_keys

*Rust Method* — `src/config/mod.rs#L816-L958`

_private_

```
fn load_provider_api_keys(config: &mut Self) -> Result<()>
```

**Calls:** parse

**Called by:** apply_env_overrides

### local_config_path

*Rust Method* — `src/config/mod.rs#L739-L741`

_private_

```
fn local_config_path() -> PathBuf
```

**Called by:** load, test_local_config_path

### merge

*Rust Method* — `src/config/mod.rs#L755-L768`

_private_

```
fn merge(_base: Self, overlay: Self) -> Self
```

**Called by:** merge_from_file

### merge_from_file

*Rust Method* — `src/config/mod.rs#L744-L752`

_private_

```
fn merge_from_file(base: Self, path: &Path) -> Result<Self>
```

**Calls:** with_context, from_str, merge

**Called by:** load, load_from_path

### project_config_path

*Rust Method* — `src/config/mod.rs#L725-L736`

```
pub fn project_config_path() -> Option<PathBuf>
```

**Called by:** load

### save

*Rust Method* — `src/config/mod.rs#L994-L1009`

```
pub fn save(&self, path: &Path) -> Result<()>
```

**Calls:** with_context

### system_config_path

*Rust Method* — `src/config/mod.rs#L720-L722`

_private_

```
fn system_config_path() -> Option<PathBuf>
```

**Called by:** load, test_system_config_path

### validate

*Rust Method* — `src/config/mod.rs#L961-L991`

```
pub fn validate(&self) -> Result<()>
```

**Calls:** is_empty

**Called by:** load_config

### default

*Rust Method* — `src/config/mod.rs#L590-L594`

_private_

```
fn default() -> Self
```

**Calls:** default_db_path

### default

*Rust Method* — `src/config/mod.rs#L616-L621`

_private_

```
fn default() -> Self
```

**Calls:** default_log_level

### default

*Rust Method* — `src/config/mod.rs#L111-L117`

_private_

```
fn default() -> Self
```

**Calls:** default_episodic_budget, default_compaction_threshold

### default

*Rust Method* — `src/config/mod.rs#L558-L571`

_private_

```
fn default() -> Self
```

**Calls:** default_ollama_host

### override_default_model

*Rust Method* — `src/config/mod.rs#L309-L341`

```
pub fn override_default_model(&mut self, model: &str) -> Option<&'static str>
```

**Called by:** run

### default

*Rust Method* — `src/config/mod.rs#L423-L440`

_private_

```
fn default() -> Self
```

### to_policy

*Rust Method* — `src/config/mod.rs#L67-L93`

```
pub fn to_policy(&self) -> Box<dyn crate::llm::tools::sandbox::PermissionPolicy>
```

**Calls:** is_empty

**Called by:** cmd_chat, cmd_run, allow_bash_trusts_only_the_listed_read_only_programs, empty_security_config_trusts_nothing

### as_str

*Rust Method* — `src/config/mod.rs#L548-L554`

```
pub fn as_str(&self) -> &str
```

### default

*Rust Method* — `src/config/mod.rs#L170-L177`

_private_

```
fn default() -> Self
```

**Calls:** default_read_file_ttl, default_glob_ttl, default_grep_ttl, default_web_search_ttl

### ttl_secs_for

*Rust Method* — `src/config/mod.rs#L182-L190`

```
pub fn ttl_secs_for(&self, tool_name: &str) -> u64
```

### allow_bash_trusts_only_the_listed_read_only_programs

*Rust Function* — `src/config/mod.rs#L1188-L1240`

_private_

```
fn allow_bash_trusts_only_the_listed_read_only_programs()
```

**Calls:** to_policy

### default

*Rust Method* — `src/config/crabrace.rs#L46-L53`

_private_

```
fn default() -> Self
```

**Calls:** default_base_url, default_auto_update, default_update_interval

### config

*Rust Method* — `src/config/crabrace.rs#L109-L111`

```
pub fn config(&self) -> &CrabraceConfig
```

### fetch_providers

*Rust Method* — `src/config/crabrace.rs#L71-L76`

```
pub async fn fetch_providers(&self) -> Result<Vec<Provider>>
```

**Called by:** get_provider, get_all_model_ids, update

### get_all_model_ids

*Rust Method* — `src/config/crabrace.rs#L93-L101`

```
pub async fn get_all_model_ids(&self) -> Result<Vec<String>>
```

**Calls:** fetch_providers

### get_provider

*Rust Method* — `src/config/crabrace.rs#L87-L90`

```
pub async fn get_provider(&self, provider_id: &str) -> Result<Option<Provider>>
```

**Calls:** fetch_providers

**Called by:** is_provider_available

### health_check

*Rust Method* — `src/config/crabrace.rs#L79-L84`

```
pub async fn health_check(&self) -> Result<bool>
```

**Called by:** test_health_check, update

### is_provider_available

*Rust Method* — `src/config/crabrace.rs#L104-L106`

```
pub async fn is_provider_available(&self, provider_id: &str) -> Result<bool>
```

**Calls:** get_provider

### new

*Rust Method* — `src/config/crabrace.rs#L64-L68`

```
pub fn new(config: CrabraceConfig) -> Result<Self>
```

### default_auto_update

*Rust Function* — `src/config/crabrace.rs#L37-L39`

_private_

```
fn default_auto_update() -> bool
```

**Called by:** default

### default_base_url

*Rust Function* — `src/config/crabrace.rs#L33-L35`

_private_

```
fn default_base_url() -> String
```

**Called by:** default

### default_enabled

*Rust Function* — `src/config/crabrace.rs#L29-L31`

_private_

```
fn default_enabled() -> bool
```

### default_update_interval

*Rust Function* — `src/config/crabrace.rs#L41-L43`

_private_

```
fn default_update_interval() -> u64
```

**Called by:** default

### test_create_integration

*Rust Function* — `src/config/crabrace.rs#L128-L132`

_private_

```
fn test_create_integration()
```

### test_default_config

*Rust Function* — `src/config/crabrace.rs#L119-L125`

_private_

```
fn test_default_config()
```

### test_health_check

*Rust Function* — `src/config/crabrace.rs#L135-L143`

_private_

```
async fn test_health_check()
```

**Calls:** health_check

### default_compaction_threshold

*Rust Function* — `src/config/mod.rs#L123-L125`

_private_

```
fn default_compaction_threshold() -> f64
```

**Called by:** default

### default_db_path

*Rust Function* — `src/config/mod.rs#L597-L602`

_private_

```
fn default_db_path() -> PathBuf
```

**Called by:** default, default

### default_enabled

*Rust Function* — `src/config/mod.rs#L578-L580`

_private_

```
fn default_enabled() -> bool
```

### default_episodic_budget

*Rust Function* — `src/config/mod.rs#L120-L122`

_private_

```
fn default_episodic_budget() -> i32
```

**Called by:** default

### default_glob_ttl

*Rust Function* — `src/config/mod.rs#L159-L161`

_private_

```
fn default_glob_ttl() -> u64
```

**Called by:** default

### default_grep_ttl

*Rust Function* — `src/config/mod.rs#L162-L164`

_private_

```
fn default_grep_ttl() -> u64
```

**Called by:** default

### default_log_level

*Rust Function* — `src/config/mod.rs#L624-L626`

_private_

```
fn default_log_level() -> String
```

**Called by:** default, default

### default_max_iterations

*Rust Function* — `src/config/mod.rs#L47-L49`

_private_

```
fn default_max_iterations() -> u32
```

### default_ollama_host

*Rust Function* — `src/config/mod.rs#L574-L576`

_private_

```
fn default_ollama_host() -> String
```

**Called by:** default

### default_read_file_ttl

*Rust Function* — `src/config/mod.rs#L156-L158`

_private_

```
fn default_read_file_ttl() -> u64
```

**Called by:** default

### default_risk_threshold

*Rust Function* — `src/config/mod.rs#L44-L46`

_private_

```
fn default_risk_threshold() -> u8
```

### default_true

*Rust Function* — `src/config/mod.rs#L126-L128`

_private_

```
fn default_true() -> bool
```

### default_web_search_ttl

*Rust Function* — `src/config/mod.rs#L165-L167`

_private_

```
fn default_web_search_ttl() -> u64
```

**Called by:** default

### empty_security_config_trusts_nothing

*Rust Function* — `src/config/mod.rs#L1245-L1254`

_private_

```
fn empty_security_config_trusts_nothing()
```

**Calls:** to_policy

### model_override_reports_when_no_provider_can_take_it

*Rust Function* — `src/config/mod.rs#L1073-L1083`

_private_

```
fn model_override_reports_when_no_provider_can_take_it()
```

### model_override_respects_provider_precedence

*Rust Function* — `src/config/mod.rs#L1044-L1068`

_private_

```
fn model_override_respects_provider_precedence()
```

### model_override_skips_disabled_providers

*Rust Function* — `src/config/mod.rs#L1089-L1114`

_private_

```
fn model_override_skips_disabled_providers()
```

### model_override_skips_gemini_without_api_key

*Rust Function* — `src/config/mod.rs#L1159-L1181`

_private_

```
fn model_override_skips_gemini_without_api_key()
```

### model_override_targets_gemini_when_it_is_the_selected_provider

*Rust Function* — `src/config/mod.rs#L1120-L1154`

_private_

```
fn model_override_targets_gemini_when_it_is_the_selected_provider()
```

### model_override_targets_the_selected_provider

*Rust Function* — `src/config/mod.rs#L1022-L1040`

_private_

```
fn model_override_targets_the_selected_provider()
```

### count

*Rust Method* — `src/config/secrets.rs#L287-L302`

```
pub fn count(&self) -> usize
```

### default

*Rust Method* — `src/config/secrets.rs#L306-L308`

_private_

```
fn default() -> Self
```

### delete_from_keyring

*Rust Method* — `src/config/secrets.rs#L265-L276`

```
pub fn delete_from_keyring(provider: &str) -> Result<()>
```

### from_env

*Rust Method* — `src/config/secrets.rs#L223-L230`

```
pub fn from_env() -> Self
```

**Calls:** from_env_optional

### has_any

*Rust Method* — `src/config/secrets.rs#L279-L284`

```
pub fn has_any(&self) -> bool
```

### load_with_fallback

*Rust Method* — `src/config/secrets.rs#L238-L245`

```
pub fn load_with_fallback() -> Self
```

### new

*Rust Method* — `src/config/secrets.rs#L213-L220`

```
pub fn new() -> Self
```

### save_to_keyring

*Rust Method* — `src/config/secrets.rs#L248-L262`

```
pub fn save_to_keyring(&self, provider: &str) -> Result<()>
```

### delete_from_keyring

*Rust Method* — `src/config/secrets.rs#L92-L102`

```
pub fn delete_from_keyring(key_name: &str) -> Result<()>
```

**Calls:** with_context

### deserialize

*Rust Method* — `src/config/secrets.rs#L179-L185`

_private_

```
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>,
```

### expose_secret

*Rust Method* — `src/config/secrets.rs#L139-L141`

```
pub fn expose_secret(&self) -> &str
```

**Called by:** save_to_keyring

### fmt

*Rust Method* — `src/config/secrets.rs#L155-L157`

_private_

```
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

### fmt

*Rust Method* — `src/config/secrets.rs#L161-L163`

_private_

```
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

### from

*Rust Method* — `src/config/secrets.rs#L195-L197`

_private_

```
fn from(s: &str) -> Self
```

**Calls:** from_str

### from

*Rust Method* — `src/config/secrets.rs#L189-L191`

_private_

```
fn from(s: String) -> Self
```

### from_env

*Rust Method* — `src/config/secrets.rs#L40-L44`

```
pub fn from_env(var_name: &str) -> Result<Self>
```

**Calls:** with_context

### from_env_optional

*Rust Method* — `src/config/secrets.rs#L47-L49`

```
pub fn from_env_optional(var_name: &str) -> Option<Self>
```

**Called by:** load_with_fallback, from_env, test_secret_string_from_env_optional

### from_keyring

*Rust Method* — `src/config/secrets.rs#L55-L64`

```
pub fn from_keyring(key_name: &str) -> Result<Self>
```

**Calls:** with_context

### from_keyring_optional

*Rust Method* — `src/config/secrets.rs#L67-L72`

```
pub fn from_keyring_optional(key_name: &str) -> Option<Self>
```

**Called by:** cmd_keyring, load_with_fallback

### from_str

*Rust Method* — `src/config/secrets.rs#L33-L37`

```
pub fn from_str(value: &str) -> Self
```

**Called by:** cmd_keyring, merge_from_file, test_config_from_toml, test_config_save_and_load, from, test_secret_string_creation, test_secret_string_debug, test_secret_string_display, test_provider_secrets_with_keys, test_secret_string_serialize, list_recent, plan_from_db, task_from_db, apply_streamed_tool_input, parse_tool_call_object, from_openai_response, stream, parse_fallback_tool_calls, from_qwen_response, load, extract_text_from_docx_xml, extract_metadata_from_core_xml, parse_xml, execute, execute, execute, load, load, match_response_line, test_update_message_metrics_with_perf_data, import_from_json, test_service_export_to_json, from

### is_empty

*Rust Method* — `src/config/secrets.rs#L144-L146`

```
pub fn is_empty(&self) -> bool
```

**Called by:** cmd_ollama, cmd_logs, to_policy, validate, update_provider_config, interrupted_plan_from_tasks, inject_into_context, summarise_turns, trim_to_fit, token_count, plan_completion_rejection, route_text_delta, apply_streamed_tool_input, drain_stream_to_response, send_message_with_tools_inner, final_text_and_thinking, extract_thinking_from_response, augment_message_with_pdf, parse_anthropic_sse_stream, ollama_provider_from_config, to_gemini_request, from_gemini_response, parse_gemini_sse, overrides_for, to_ollama_request, from_ollama_response, stream, stop_reason_for, maybe_tool_call_json, to_openai_request, from_openai_response, stream, parse_fallback_tool_calls, parse_native_qwen_tool_calls, to_qwen_request, push_fallback_or_text, from_qwen_response, stream, llm_response_to_stream_events, extract_think_tags, validate_input, execute, slugify, parse_patch, find_subsequence, apply_hunks, validate_input, execute, validate_input, execute, validate_input, execute, execute, parse_pdf, extract_text_from_docx_xml, strip_html_tags, parse_xml, validate_input, execute, validate_input, execute, execute, validate_string, execute, validate_input, execute, read_with_buffer, append_fact, validate_input, execute, validate_input, execute, parse_skill_frontmatter_value, resolve, execute, render_todos, validate_input, execute, init_debug_logging, tasks_in_order, progress_percentage, is_complete, validate_dependencies, get_validation_warnings, validate_plan, get_statistics, input_is_blank, history_prev, handle_chat_key, handle_skills_key, handle_mcp_key, export_plan_to_markdown, start_model_pull, handle_model_download_key, handle_provider_switch_key, is_submit, is_enter, is_up, is_down, is_approve, is_deny, is_view_details, flush_current_line, start_code_block, end_heading, end_code_block, finish, filter_suggestions, analyze_and_transform, render_skills, render_mcp, render_plan_task_lines, render_plan_document, render_approval_capabilities, render_approval_input_summary, render_provider_switch, render_model_download

### len

*Rust Method* — `src/config/secrets.rs#L149-L151`

```
pub fn len(&self) -> usize
```

**Called by:** read_sequential, cmd_logs, interrupted_plan_from_tasks, list_recent, compact, compaction_atomicity_db_failure_leaves_context_unchanged, token_count, test_trim_to_fit, route_text_delta, send_message_with_tools_inner, augment_message_with_pdf, complete, stream, parse_keep_alive, complete, stream, find_json_objects, expand_span_over_adjacent_fences, parse_native_qwen_tool_calls, clean_incomplete_markers, from_qwen_response, complete, stream, extract_think_tags, parse_patch, find_subsequence, apply_hunks, execute, is_read_only_command, execute, execute, parse_pdf, strip_html_tags, execute, of, execute, execute, search_file, execute, list_directory, execute, validate_plan_file_path, validate_string, execute, execute, count, checked_redirect_policy, execute, execute, execute, execute, read_response_line, tasks_in_order, progress_percentage, execution_summary, get_validation_warnings, approve, validate_plan, get_statistics, cursor_on_last_line, history_prev, history_next, handle_sessions_key, handle_skills_key, handle_mcp_key, handle_plan_key, check_and_load_plan, execute_next_plan_task, handle_file_picker_key, handle_model_download_key, handle_provider_switch_key, stale_session_response_complete_is_dropped_after_switching_sessions, send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight, send_message_still_works_for_a_different_session_than_the_one_processing, render_header, render_processing_indicator, render_chat, render_approval_input_summary, render_file_picker, truncate_at_char_boundary, compaction_fails_gracefully_with_insufficient_turns, complete

### load_with_fallback

*Rust Method* — `src/config/secrets.rs#L110-L132`

```
pub fn load_with_fallback(key_name: &str, env_var: &str) -> Option<Self>
```

**Calls:** from_keyring_optional, from_env_optional

### new

*Rust Method* — `src/config/secrets.rs#L27-L29`

```
pub fn new(value: String) -> Self
```

### save_to_keyring

*Rust Method* — `src/config/secrets.rs#L79-L89`

```
pub fn save_to_keyring(&self, key_name: &str) -> Result<()>
```

**Calls:** with_context, expose_secret

### serialize

*Rust Method* — `src/config/secrets.rs#L168-L174`

_private_

```
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer,
```

### test_provider_secrets_empty

*Rust Function* — `src/config/secrets.rs#L340-L344`

_private_

```
fn test_provider_secrets_empty()
```

### test_provider_secrets_with_keys

*Rust Function* — `src/config/secrets.rs#L347-L356`

_private_

```
fn test_provider_secrets_with_keys()
```

**Calls:** from_str

### test_secret_string_creation

*Rust Function* — `src/config/secrets.rs#L316-L321`

_private_

```
fn test_secret_string_creation()
```

**Calls:** from_str

### test_secret_string_debug

*Rust Function* — `src/config/secrets.rs#L324-L329`

_private_

```
fn test_secret_string_debug()
```

**Calls:** from_str

### test_secret_string_display

*Rust Function* — `src/config/secrets.rs#L332-L337`

_private_

```
fn test_secret_string_display()
```

**Calls:** from_str

### test_secret_string_from_env

*Rust Function* — `src/config/secrets.rs#L359-L368`

_private_

```
fn test_secret_string_from_env()
```

### test_secret_string_from_env_optional

*Rust Function* — `src/config/secrets.rs#L371-L384`

_private_

```
fn test_secret_string_from_env_optional()
```

**Calls:** from_env_optional

### test_secret_string_serialize

*Rust Function* — `src/config/secrets.rs#L387-L392`

_private_

```
fn test_secret_string_serialize()
```

**Calls:** from_str

### test_config_env_overrides

*Rust Function* — `src/config/mod.rs#L1330-L1350`

_private_

```
fn test_config_env_overrides()
```

**Calls:** apply_env_overrides

### test_config_from_toml

*Rust Function* — `src/config/mod.rs#L1286-L1311`

_private_

```
fn test_config_from_toml()
```

**Calls:** from_str

### test_config_save_and_load

*Rust Function* — `src/config/mod.rs#L1314-L1327`

_private_

```
fn test_config_save_and_load()
```

**Calls:** from_str

### test_config_validation

*Rust Function* — `src/config/mod.rs#L1266-L1269`

_private_

```
fn test_config_validation()
```

### test_config_validation_empty_crabrace_url

*Rust Function* — `src/config/mod.rs#L1279-L1283`

_private_

```
fn test_config_validation_empty_crabrace_url()
```

### test_config_validation_invalid_log_level

*Rust Function* — `src/config/mod.rs#L1272-L1276`

_private_

```
fn test_config_validation_invalid_log_level()
```

### test_database_config_default

*Rust Function* — `src/config/mod.rs#L1465-L1468`

_private_

```
fn test_database_config_default()
```

### test_debug_config_default

*Rust Function* — `src/config/mod.rs#L1447-L1451`

_private_

```
fn test_debug_config_default()
```

### test_default_config

*Rust Function* — `src/config/mod.rs#L1257-L1263`

_private_

```
fn test_default_config()
```

### test_local_config_path

*Rust Function* — `src/config/mod.rs#L1441-L1444`

_private_

```
fn test_local_config_path()
```

**Calls:** local_config_path

### test_logging_config_default

*Rust Function* — `src/config/mod.rs#L1471-L1475`

_private_

```
fn test_logging_config_default()
```

### test_ollama_config_from_env

*Rust Function* — `src/config/mod.rs#L1384-L1419`

_private_

```
fn test_ollama_config_from_env()
```

**Calls:** apply_env_overrides

### test_ollama_provider_config_default

*Rust Function* — `src/config/mod.rs#L1422-L1429`

_private_

```
fn test_ollama_provider_config_default()
```

### test_provider_config_api_keys_from_env

*Rust Function* — `src/config/mod.rs#L1353-L1381`

_private_

```
fn test_provider_config_api_keys_from_env()
```

**Calls:** apply_env_overrides

### test_provider_configs_default

*Rust Function* — `src/config/mod.rs#L1454-L1462`

_private_

```
fn test_provider_configs_default()
```

### test_system_config_path

*Rust Function* — `src/config/mod.rs#L1432-L1438`

_private_

```
fn test_system_config_path()
```

**Calls:** system_config_path

### new

*Rust Method* — `src/config/update.rs#L23-L28`

```
pub fn new(crabrace: CrabraceIntegration) -> Self
```

### should_update

*Rust Method* — `src/config/update.rs#L31-L47`

```
pub fn should_update(&self, config: &Config) -> bool
```

**Called by:** start_auto_update_loop

### start_auto_update_loop

*Rust Method* — `src/config/update.rs#L156-L181`

```
pub async fn start_auto_update_loop(mut self, mut config: Config)
```

**Calls:** should_update

### update

*Rust Method* — `src/config/update.rs#L50-L106`

```
pub async fn update(&mut self, config: &mut Config) -> Result<UpdateResult>
```

**Calls:** health_check, fetch_providers, update_provider_config

### update_once

*Rust Method* — `src/config/update.rs#L184-L188`

```
pub async fn update_once(config: &mut Config) -> Result<UpdateResult>
```

### update_provider_config

*Rust Method* — `src/config/update.rs#L109-L153`

_private_

```
fn update_provider_config(&self, config: &mut Config, provider: &Provider) -> bool
```

**Calls:** is_empty

**Called by:** update

### failure

*Rust Method* — `src/config/update.rs#L213-L219`

```
pub fn failure(error: String) -> Self
```

**Called by:** test_update_result_failure

### success

*Rust Method* — `src/config/update.rs#L204-L210`

```
pub fn success(providers_updated: usize) -> Self
```

### test_should_update_when_disabled

*Rust Function* — `src/config/update.rs#L228-L242`

_private_

```
fn test_should_update_when_disabled()
```

### test_should_update_when_never_updated

*Rust Function* — `src/config/update.rs#L245-L260`

_private_

```
fn test_should_update_when_never_updated()
```

### test_update_result_failure

*Rust Function* — `src/config/update.rs#L271-L276`

_private_

```
fn test_update_result_failure()
```

**Calls:** failure

### test_update_result_success

*Rust Function* — `src/config/update.rs#L263-L268`

_private_

```
fn test_update_result_success()
```

### close

*Rust Method* — `src/db/mod.rs#L177-L181`

```
pub async fn close(self) -> Result<()>
```

### connect

*Rust Method* — `src/db/mod.rs#L34-L90`

```
pub async fn connect<P: AsRef<Path>>(path: P) -> Result<Self>
```

**Calls:** with_context

### connect_in_memory

*Rust Method* — `src/db/mod.rs#L93-L112`

```
pub async fn connect_in_memory() -> Result<Self>
```

### is_connected

*Rust Method* — `src/db/mod.rs#L120-L122`

```
pub fn is_connected(&self) -> bool
```

### pool

*Rust Method* — `src/db/mod.rs#L115-L117`

```
pub fn pool(&self) -> &SqlitePool
```

### run_migrations

*Rust Method* — `src/db/mod.rs#L148-L174`

```
pub async fn run_migrations(&self) -> Result<()>
```

**Calls:** acquire

**Called by:** setup_test_db, cmd_db, cmd_chat, cmd_run, foreign_keys_are_enforced, deleting_a_session_cascades_to_its_messages, migrating_from_pre_modernization_schema_preserves_existing_messages, test_file_crud, test_file_list_by_session, test_message_crud, test_message_list_by_session, setup_test_db, test_session_crud, test_session_archive, create_test_service, sub_agent_launcher_does_not_auto_approve_tools, test_send_message_with_tool_execution, create_test_service, create_test_service, create_then_update_survives_a_file_backed_wal_pool, create_test_pool, setup_test_service, create_test_service, test_app, test_app, run_loop_exits_immediately_when_should_quit_is_set, compaction_preserves_last_10_turns, compaction_fails_gracefully_with_insufficient_turns, compaction_writes_one_record_to_db, create_test_db, create_test_db, test_database_persistence, crash_recovery_resumes_at_correct_task, task_state_transitions_correct_order, failed_task_stores_error_without_completion_timestamp, setup_test_env

### connect_file

*Rust Method* — `src/db/mod.rs#L202-L205`

_private_

```
async fn connect_file<P: AsRef<Path>>(path: P) -> Result<Self>
```

### connect_in_memory

*Rust Method* — `src/db/mod.rs#L207-L210`

_private_

```
async fn connect_in_memory() -> Result<Self>
```

### is_connected

*Rust Method* — `src/db/mod.rs#L212-L214`

_private_

```
fn is_connected(&self) -> bool
```

### deleting_a_session_cascades_to_its_messages

*Rust Function* — `src/db/mod.rs#L303-L346`

_private_

```
async fn deleting_a_session_cascades_to_its_messages()
```

**Calls:** run_migrations

### foreign_keys_are_enforced

*Rust Function* — `src/db/mod.rs#L274-L296`

_private_

```
async fn foreign_keys_are_enforced()
```

**Calls:** run_migrations

### migrating_from_pre_modernization_schema_preserves_existing_messages

*Rust Function* — `src/db/mod.rs#L369-L439`

_private_

```
async fn migrating_from_pre_modernization_schema_preserves_existing_messages()
```

**Calls:** run_migrations

### new

*Rust Method* — `src/db/models.rs#L261-L271`

```
pub fn new(session_id: Uuid, path: std::path::PathBuf, content: Option<String>) -> Self
```

### from_row

*Rust Method* — `src/db/models.rs#L321-L336`

_private_

```
fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error>
```

### new

*Rust Method* — `src/db/models.rs#L243-L256`

```
pub fn new(session_id: Uuid, role: String, content: String, sequence: i32) -> Self
```

### from_row

*Rust Method* — `src/db/models.rs#L299-L317`

_private_

```
fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error>
```

### from_row

*Rust Method* — `src/db/models.rs#L340-L363`

_private_

```
fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error>
```

### exec_status

*Rust Method* — `src/db/models.rs#L183-L185`

```
pub fn exec_status(&self) -> PlanTaskStatus
```

**Calls:** parse

**Called by:** interrupted_plan_from_tasks

### from_row

*Rust Method* — `src/db/models.rs#L367-L394`

_private_

```
fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error>
```

### task_index

*Rust Method* — `src/db/models.rs#L178-L180`

```
pub fn task_index(&self) -> i32
```

### as_str

*Rust Method* — `src/db/models.rs#L123-L131`

```
pub fn as_str(&self) -> &'static str
```

### is_incomplete

*Rust Method* — `src/db/models.rs#L150-L152`

```
pub fn is_incomplete(&self) -> bool
```

**Called by:** interrupted_plan_from_tasks

### parse

*Rust Method* — `src/db/models.rs#L133-L148`

```
pub fn parse(s: &str) -> Self
```

**Called by:** run, apply_env_overrides, load_provider_api_keys, exec_status, list_for_session, list_recent, row_to_plan_task, row_to_entry, headers, headers, headers, headers, execute, check_url_not_blocked, init_debug_logging, init_minimal_logging, debug_filter_is_scoped_to_crustly, main

### is_archived

*Rust Method* — `src/db/models.rs#L236-L238`

```
pub fn is_archived(&self) -> bool
```

### new

*Rust Method* — `src/db/models.rs#L220-L233`

```
pub fn new(title: Option<String>, model: Option<String>) -> Self
```

### from_row

*Rust Method* — `src/db/models.rs#L276-L295`

_private_

```
fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error>
```

### interrupted_plan_from_tasks

*Rust Function* — `src/db/models.rs#L195-L216`

```
pub fn interrupted_plan_from_tasks( plan_id: Uuid, tasks: &[PlanTask], ) -> Option<crate::plan::InterruptedPlan>
```

**Calls:** exec_status, is_incomplete, is_empty, len

**Called by:** interrupted_plan_none_when_all_done, interrupted_plan_resumes_at_lowest_incomplete

### test_file_new

*Rust Function* — `src/db/models.rs#L427-L435`

_private_

```
fn test_file_new()
```

### test_message_new

*Rust Function* — `src/db/models.rs#L415-L424`

_private_

```
fn test_message_new()
```

### test_session_archived

*Rust Function* — `src/db/models.rs#L438-L445`

_private_

```
fn test_session_archived()
```

### test_session_new

*Rust Function* — `src/db/models.rs#L402-L412`

_private_

```
fn test_session_new()
```

### insert

*Rust Method* — `src/db/repository/compaction.rs#L18-L36`

```
pub async fn insert(&self, rec: &CompactionRecord) -> Result<()>
```

### list_for_session

*Rust Method* — `src/db/repository/compaction.rs#L38-L64`

```
pub async fn list_for_session(&self, session_id: Uuid) -> Result<Vec<CompactionRecord>>
```

**Calls:** parse

**Called by:** compaction_writes_one_record_to_db

### new

*Rust Method* — `src/db/repository/compaction.rs#L14-L16`

```
pub fn new(pool: SqlitePool) -> Self
```

### count_by_session

*Rust Method* — `src/db/repository/file.rs#L126-L134`

```
pub async fn count_by_session(&self, session_id: Uuid) -> Result<i64>
```

### create

*Rust Method* — `src/db/repository/file.rs#L62-L83`

```
pub async fn create(&self, file: &File) -> Result<()>
```

### delete

*Rust Method* — `src/db/repository/file.rs#L109-L118`

```
pub async fn delete(&self, id: Uuid) -> Result<()>
```

### delete_by_session

*Rust Method* — `src/db/repository/file.rs#L137-L146`

```
pub async fn delete_by_session(&self, session_id: Uuid) -> Result<()>
```

### find_by_id

*Rust Method* — `src/db/repository/file.rs#L24-L32`

```
pub async fn find_by_id(&self, id: Uuid) -> Result<Option<File>>
```

### find_by_path

*Rust Method* — `src/db/repository/file.rs#L48-L59`

```
pub async fn find_by_path(&self, session_id: Uuid, path: &Path) -> Result<Option<File>>
```

**Called by:** find_file_by_path

### find_by_session

*Rust Method* — `src/db/repository/file.rs#L35-L45`

```
pub async fn find_by_session(&self, session_id: Uuid) -> Result<Vec<File>>
```

### list_by_session

*Rust Method* — `src/db/repository/file.rs#L121-L123`

```
pub async fn list_by_session(&self, session_id: Uuid) -> Result<Vec<File>>
```

### new

*Rust Method* — `src/db/repository/file.rs#L19-L21`

```
pub fn new(pool: SqlitePool) -> Self
```

### update

*Rust Method* — `src/db/repository/file.rs#L86-L106`

```
pub async fn update(&self, file: &File) -> Result<()>
```

### test_file_crud

*Rust Function* — `src/db/repository/file.rs#L158-L197`

_private_

```
async fn test_file_crud()
```

**Calls:** run_migrations

### test_file_list_by_session

*Rust Function* — `src/db/repository/file.rs#L200-L238`

_private_

```
async fn test_file_list_by_session()
```

**Calls:** run_migrations

### inject_into_context

*Rust Method* — `src/db/repository/memory.rs#L99-L130`

```
pub async fn inject_into_context( &self, ctx: &mut crate::llm::agent::context::AgentContext, max_tokens: i32, ) -> Result<()>
```

**Calls:** list_recent, is_empty

**Called by:** inject_episodic_memories

### insert

*Rust Method* — `src/db/repository/memory.rs#L19-L38`

```
pub async fn insert(&self, mem: EpisodicMemory) -> Result<()>
```

### list_recent

*Rust Method* — `src/db/repository/memory.rs#L41-L96`

```
pub async fn list_recent(&self, limit: u32, max_tokens: i32) -> Result<Vec<EpisodicMemory>>
```

**Calls:** len, parse, from_str

**Called by:** inject_into_context, list_recent_truncates_multibyte_summary_without_panicking

### new

*Rust Method* — `src/db/repository/memory.rs#L14-L16`

```
pub fn new(pool: SqlitePool) -> Self
```

### create_test_pool

*Rust Function* — `src/db/repository/memory.rs#L139-L156`

_private_

```
async fn create_test_pool() -> sqlx::SqlitePool
```

### episodic_memory_inject_3_memories_within_budget

*Rust Function* — `src/db/repository/memory.rs#L159-L221`

_private_

```
async fn episodic_memory_inject_3_memories_within_budget()
```

**Calls:** token_count, inject_episodic_memories

### list_recent_truncates_multibyte_summary_without_panicking

*Rust Function* — `src/db/repository/memory.rs#L232-L253`

_private_

```
async fn list_recent_truncates_multibyte_summary_without_panicking()
```

**Calls:** list_recent

### count_by_session

*Rust Method* — `src/db/repository/message.rs#L165-L173`

```
pub async fn count_by_session(&self, session_id: Uuid) -> Result<i64>
```

### create

*Rust Method* — `src/db/repository/message.rs#L59-L122`

```
pub async fn create(&self, message: &mut Message) -> Result<()>
```

### delete

*Rust Method* — `src/db/repository/message.rs#L148-L157`

```
pub async fn delete(&self, id: Uuid) -> Result<()>
```

### delete_by_session

*Rust Method* — `src/db/repository/message.rs#L189-L198`

```
pub async fn delete_by_session(&self, session_id: Uuid) -> Result<()>
```

### find_by_id

*Rust Method* — `src/db/repository/message.rs#L23-L31`

```
pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Message>>
```

### find_by_session

*Rust Method* — `src/db/repository/message.rs#L34-L44`

```
pub async fn find_by_session(&self, session_id: Uuid) -> Result<Vec<Message>>
```

### get_last_message

*Rust Method* — `src/db/repository/message.rs#L176-L186`

```
pub async fn get_last_message(&self, session_id: Uuid) -> Result<Option<Message>>
```

### list_by_session

*Rust Method* — `src/db/repository/message.rs#L160-L162`

```
pub async fn list_by_session(&self, session_id: Uuid) -> Result<Vec<Message>>
```

### new

*Rust Method* — `src/db/repository/message.rs#L18-L20`

```
pub fn new(pool: SqlitePool) -> Self
```

### update

*Rust Method* — `src/db/repository/message.rs#L125-L145`

```
pub async fn update(&self, message: &Message) -> Result<()>
```

### test_message_crud

*Rust Function* — `src/db/repository/message.rs#L209-L263`

_private_

```
async fn test_message_crud()
```

**Calls:** run_migrations

### test_message_list_by_session

*Rust Function* — `src/db/repository/message.rs#L266-L305`

_private_

```
async fn test_message_list_by_session()
```

**Calls:** run_migrations

### create

*Rust Method* — `src/db/repository/plan.rs#L82-L142`

```
pub async fn create(&self, plan: &PlanDocument) -> Result<()>
```

**Calls:** plan_to_db

### delete

*Rust Method* — `src/db/repository/plan.rs#L248-L258`

```
pub async fn delete(&self, id: Uuid) -> Result<()>
```

### find_by_id

*Rust Method* — `src/db/repository/plan.rs#L31-L47`

```
pub async fn find_by_id(&self, id: Uuid) -> Result<Option<PlanDocument>>
```

**Calls:** find_tasks_by_plan_id, plan_from_db

### find_by_session_id

*Rust Method* — `src/db/repository/plan.rs#L50-L66`

```
pub async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<PlanDocument>>
```

**Calls:** find_tasks_by_plan_id, plan_from_db

### find_tasks_by_plan_id

*Rust Method* — `src/db/repository/plan.rs#L69-L79`

_private_

```
async fn find_tasks_by_plan_id(&self, plan_id: Uuid) -> Result<Vec<PlanTask>>
```

**Called by:** find_by_id, find_by_session_id

### format_plan_status

*Rust Method* — `src/db/repository/plan.rs#L392-L403`

_private_

```
fn format_plan_status(&self, status: &PlanStatus) -> String
```

**Called by:** plan_to_db

### format_task_status

*Rust Method* — `src/db/repository/plan.rs#L462-L471`

_private_

```
fn format_task_status(&self, status: &TaskStatus) -> String
```

**Called by:** task_to_db

### format_task_type

*Rust Method* — `src/db/repository/plan.rs#L422-L436`

_private_

```
fn format_task_type(&self, task_type: &TaskType) -> String
```

**Called by:** task_to_db

### new

*Rust Method* — `src/db/repository/plan.rs#L26-L28`

```
pub fn new(pool: SqlitePool) -> Self
```

### parse_plan_status

*Rust Method* — `src/db/repository/plan.rs#L378-L389`

_private_

```
fn parse_plan_status(&self, status: &str) -> Result<PlanStatus>
```

**Called by:** plan_from_db

### parse_task_status

*Rust Method* — `src/db/repository/plan.rs#L439-L459`

_private_

```
fn parse_task_status(&self, status: &str) -> Result<TaskStatus>
```

**Called by:** task_from_db

### parse_task_type

*Rust Method* — `src/db/repository/plan.rs#L406-L419`

_private_

```
fn parse_task_type(&self, task_type: &str) -> Result<TaskType>
```

**Called by:** task_from_db

### plan_from_db

*Rust Method* — `src/db/repository/plan.rs#L261-L289`

_private_

```
fn plan_from_db(&self, db_plan: Plan, db_tasks: Vec<PlanTask>) -> Result<PlanDocument>
```

**Calls:** from_str, parse_plan_status, task_from_db

**Called by:** find_by_id, find_by_session_id

### plan_to_db

*Rust Method* — `src/db/repository/plan.rs#L323-L349`

_private_

```
fn plan_to_db(&self, plan: &PlanDocument) -> Result<(Plan, Vec<PlanTask>)>
```

**Calls:** format_plan_status, task_to_db

**Called by:** create, update

### task_from_db

*Rust Method* — `src/db/repository/plan.rs#L292-L320`

_private_

```
fn task_from_db(&self, db_task: PlanTask) -> Result<crate::plan::PlanTask>
```

**Calls:** from_str, parse_task_type, parse_task_status

**Called by:** plan_from_db

### task_to_db

*Rust Method* — `src/db/repository/plan.rs#L352-L375`

_private_

```
fn task_to_db(&self, task: &crate::plan::PlanTask, plan_id: Uuid) -> Result<PlanTask>
```

**Calls:** format_task_type, format_task_status

**Called by:** plan_to_db

### update

*Rust Method* — `src/db/repository/plan.rs#L145-L245`

```
pub async fn update(&self, plan: &PlanDocument) -> Result<()>
```

**Calls:** plan_to_db

### create_task

*Rust Method* — `src/db/repository/plan.rs#L492-L514`

```
pub async fn create_task(&self, task: PlanTask) -> Result<()>
```

**Called by:** crash_recovery_resumes_at_correct_task, task_state_transitions_correct_order, failed_task_stores_error_without_completion_timestamp

### get_incomplete_tasks

*Rust Method* — `src/db/repository/plan.rs#L620-L650`

```
pub async fn get_incomplete_tasks(&self, plan_id: Uuid) -> Result<Vec<PlanTask>>
```

### get_task

*Rust Method* — `src/db/repository/plan.rs#L558-L586`

```
pub async fn get_task(&self, task_id: Uuid) -> Result<PlanTask>
```

**Calls:** row_to_plan_task

### list_tasks_for_plan

*Rust Method* — `src/db/repository/plan.rs#L589-L617`

```
pub async fn list_tasks_for_plan(&self, plan_id: Uuid) -> Result<Vec<PlanTask>>
```

### new

*Rust Method* — `src/db/repository/plan.rs#L487-L489`

```
pub fn new(pool: SqlitePool) -> Self
```

### update_task_status

*Rust Method* — `src/db/repository/plan.rs#L519-L555`

```
pub async fn update_task_status( &self, task_id: Uuid, status: PlanTaskStatus, output_summary: Option<String>, error_text: Option<String>, ) -> Result<()>
```

**Called by:** begin_task, complete_task, fail_task, task_state_transitions_correct_order, failed_task_stores_error_without_completion_timestamp

### create_test_plan

*Rust Function* — `src/db/repository/plan.rs#L743-L796`

_private_

```
fn create_test_plan(session_id: Uuid) -> PlanDocument
```

**Calls:** add_task

### row_to_plan_task

*Rust Function* — `src/db/repository/plan.rs#L654-L708`

_private_

```
fn row_to_plan_task( row: ( String, String, i32, String, String, String, String, i32, String, String, Option<String>, Option<i64>, Option<i64>, Option<String>, Option<String>, ), ) -> PlanTask
```

**Calls:** parse

**Called by:** get_task

### setup_test_db

*Rust Function* — `src/db/repository/plan.rs#L720-L740`

_private_

```
async fn setup_test_db() -> (Database, SessionRepository, PlanRepository, Session)
```

**Calls:** run_migrations

### test_multiple_sessions_multiple_plans

*Rust Function* — `src/db/repository/plan.rs#L1272-L1316`

_private_

```
async fn test_multiple_sessions_multiple_plans()
```

### test_plan_create

*Rust Function* — `src/db/repository/plan.rs#L799-L819`

_private_

```
async fn test_plan_create()
```

### test_plan_delete

*Rust Function* — `src/db/repository/plan.rs#L929-L954`

_private_

```
async fn test_plan_delete()
```

### test_plan_find_by_id

*Rust Function* — `src/db/repository/plan.rs#L822-L844`

_private_

```
async fn test_plan_find_by_id()
```

### test_plan_find_by_session_id

*Rust Function* — `src/db/repository/plan.rs#L847-L873`

_private_

```
async fn test_plan_find_by_session_id()
```

### test_plan_risks_serialization

*Rust Function* — `src/db/repository/plan.rs#L1121-L1139`

_private_

```
async fn test_plan_risks_serialization()
```

### test_plan_status_conversion

*Rust Function* — `src/db/repository/plan.rs#L988-L1020`

_private_

```
async fn test_plan_status_conversion()
```

### test_plan_tasks_cascade_delete

*Rust Function* — `src/db/repository/plan.rs#L957-L985`

_private_

```
async fn test_plan_tasks_cascade_delete()
```

### test_plan_update

*Rust Function* — `src/db/repository/plan.rs#L876-L926`

_private_

```
async fn test_plan_update()
```

**Calls:** add_task

### test_plan_update_task_status

*Rust Function* — `src/db/repository/plan.rs#L1168-L1204`

_private_

```
async fn test_plan_update_task_status()
```

**Calls:** get_task_mut

### test_plan_with_complex_task_graph

*Rust Function* — `src/db/repository/plan.rs#L1207-L1269`

_private_

```
async fn test_plan_with_complex_task_graph()
```

**Calls:** add_task

### test_plan_with_no_tasks

*Rust Function* — `src/db/repository/plan.rs#L1142-L1165`

_private_

```
async fn test_plan_with_no_tasks()
```

### test_task_dependencies_serialization

*Rust Function* — `src/db/repository/plan.rs#L1095-L1118`

_private_

```
async fn test_task_dependencies_serialization()
```

### test_task_status_conversion

*Rust Function* — `src/db/repository/plan.rs#L1061-L1092`

_private_

```
async fn test_task_status_conversion()
```

### test_task_type_conversion

*Rust Function* — `src/db/repository/plan.rs#L1023-L1058`

_private_

```
async fn test_task_type_conversion()
```

### archive

*Rust Method* — `src/db/repository/session.rs#L173-L186`

```
pub async fn archive(&self, id: Uuid) -> Result<()>
```

**Called by:** test_session_archive, archive_session

### count

*Rust Method* — `src/db/repository/session.rs#L228-L241`

```
pub async fn count(&self, archived_only: bool) -> Result<i64>
```

### create

*Rust Method* — `src/db/repository/session.rs#L46-L69`

```
pub async fn create(&self, session: &Session) -> Result<()>
```

### delete

*Rust Method* — `src/db/repository/session.rs#L98-L107`

```
pub async fn delete(&self, id: Uuid) -> Result<()>
```

### find_by_id

*Rust Method* — `src/db/repository/session.rs#L35-L43`

```
pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>>
```

### list

*Rust Method* — `src/db/repository/session.rs#L110-L146`

```
pub async fn list(&self, options: SessionListOptions) -> Result<Vec<Session>>
```

**Called by:** list_sessions, get_most_recent_session

### list_active

*Rust Method* — `src/db/repository/session.rs#L149-L158`

```
pub async fn list_active(&self) -> Result<Vec<Session>>
```

### list_archived

*Rust Method* — `src/db/repository/session.rs#L161-L170`

```
pub async fn list_archived(&self) -> Result<Vec<Session>>
```

### new

*Rust Method* — `src/db/repository/session.rs#L30-L32`

```
pub fn new(pool: SqlitePool) -> Self
```

### unarchive

*Rust Method* — `src/db/repository/session.rs#L189-L201`

```
pub async fn unarchive(&self, id: Uuid) -> Result<()>
```

**Called by:** test_session_archive, unarchive_session

### update

*Rust Method* — `src/db/repository/session.rs#L72-L95`

```
pub async fn update(&self, session: &Session) -> Result<()>
```

### update_stats

*Rust Method* — `src/db/repository/session.rs#L204-L225`

```
pub async fn update_stats(&self, id: Uuid, token_delta: i32, cost_delta: f64) -> Result<()>
```

### test_session_archive

*Rust Function* — `src/db/repository/session.rs#L302-L333`

_private_

```
async fn test_session_archive()
```

**Calls:** run_migrations, archive, unarchive

### test_session_crud

*Rust Function* — `src/db/repository/session.rs#L250-L299`

_private_

```
async fn test_session_crud()
```

**Calls:** run_migrations

### aggressive

*Rust Method* — `src/db/retry.rs#L51-L58`

```
pub fn aggressive() -> Self
```

### calculate_delay

*Rust Method* — `src/db/retry.rs#L61-L68`

_private_

```
fn calculate_delay(&self, attempt: u32) -> Duration
```

### default

*Rust Method* — `src/db/retry.rs#L30-L37`

_private_

```
fn default() -> Self
```

### new

*Rust Method* — `src/db/retry.rs#L42-L48`

```
pub fn new(max_attempts: u32, initial_delay: Duration) -> Self
```

### is_database_locked

*Rust Function* — `src/db/retry.rs#L72-L80`

_private_

```
fn is_database_locked(err: &sqlx::Error) -> bool
```

**Called by:** retry_db_sqlx

### retry_db_anyhow

*Rust Function* — `src/db/retry.rs#L164-L172`

```
pub async fn retry_db_anyhow<F, Fut, T>(operation: F, config: &DbRetryConfig) -> Result<T> where F: FnMut() -> Fut, Fut: Future<Output = Result<T>>,
```

**Calls:** retry_db_operation

### retry_db_operation

*Rust Function* — `src/db/retry.rs#L100-L161`

```
pub async fn retry_db_operation<F, Fut, T, E>( mut operation: F, config: &DbRetryConfig, ) -> std::result::Result<T, E> where F: FnMut() -> Fut, Fut: Future<Output = std::result::Result<T, E>>, E: std::fmt::Display,
```

**Called by:** retry_db_anyhow, test_retry_success_immediate, test_retry_success_after_retries, test_retry_max_attempts_exceeded, test_retry_non_retryable_error

### retry_db_sqlx

*Rust Function* — `src/db/retry.rs#L175-L233`

```
pub async fn retry_db_sqlx<F, Fut, T>( mut operation: F, config: &DbRetryConfig, ) -> std::result::Result<T, sqlx::Error> where F: FnMut() -> Fut, Fut: Future<Output = std::result::Result<T, sqlx::Error>>,
```

**Calls:** is_database_locked

### test_calculate_delay

*Rust Function* — `src/db/retry.rs#L255-L275`

_private_

```
fn test_calculate_delay()
```

### test_is_database_locked

*Rust Function* — `src/db/retry.rs#L278-L288`

_private_

```
fn test_is_database_locked()
```

### test_retry_config_aggressive

*Rust Function* — `src/db/retry.rs#L248-L252`

_private_

```
fn test_retry_config_aggressive()
```

### test_retry_config_defaults

*Rust Function* — `src/db/retry.rs#L240-L245`

_private_

```
fn test_retry_config_defaults()
```

### test_retry_max_attempts_exceeded

*Rust Function* — `src/db/retry.rs#L347-L369`

_private_

```
async fn test_retry_max_attempts_exceeded()
```

**Calls:** retry_db_operation

### test_retry_non_retryable_error

*Rust Function* — `src/db/retry.rs#L372-L394`

_private_

```
async fn test_retry_non_retryable_error()
```

**Calls:** retry_db_operation

### test_retry_success_after_retries

*Rust Function* — `src/db/retry.rs#L317-L344`

_private_

```
async fn test_retry_success_after_retries()
```

**Calls:** retry_db_operation

### test_retry_success_immediate

*Rust Function* — `src/db/retry.rs#L291-L314`

_private_

```
async fn test_retry_success_immediate()
```

**Calls:** retry_db_operation

### test_connect_in_memory

*Rust Function* — `src/db/mod.rs#L222-L225`

_private_

```
async fn test_connect_in_memory()
```

### test_pool_connect_in_memory

*Rust Function* — `src/db/mod.rs#L228-L231`

_private_

```
async fn test_pool_connect_in_memory()
```

### tilde_in_the_database_path_is_expanded_to_home

*Rust Function* — `src/db/mod.rs#L239-L264`

_private_

```
async fn tilde_in_the_database_path_is_expanded_to_home()
```

**Calls:** drop

### code

*Rust Method* — `src/error.rs#L56-L63`

```
pub fn code(&self) -> Option<ErrorCode>
```

**Called by:** execute, execute, execute

### user_message

*Rust Method* — `src/error.rs#L65-L92`

```
pub fn user_message(&self) -> String
```

### compact

*Rust Function* — `src/llm/agent/compaction.rs#L27-L124`

```
pub async fn compact(ctx: &mut AgentContext, pool: &sqlx::SqlitePool) -> Result<CompactionRecord>
```

**Calls:** len, message_has_tool_result, summarise_turns, token_count

**Called by:** compaction_atomicity_db_failure_leaves_context_unchanged, compaction_integration_preserves_last_10_turns, compaction_never_splits_a_tool_use_result_pair, send_message_with_tools_inner, init_minimal_logging, compaction_preserves_last_10_turns, compaction_fails_gracefully_with_insufficient_turns, compaction_writes_one_record_to_db

### compaction_atomicity_db_failure_leaves_context_unchanged

*Rust Function* — `src/llm/agent/compaction.rs#L218-L255`

_private_

```
async fn compaction_atomicity_db_failure_leaves_context_unchanged()
```

**Calls:** add_message, len, compact

### compaction_fires_at_threshold

*Rust Function* — `src/llm/agent/compaction.rs#L181-L215`

_private_

```
fn compaction_fires_at_threshold()
```

**Calls:** add_message, should_compact, usage_percentage

### compaction_integration_preserves_last_10_turns

*Rust Function* — `src/llm/agent/compaction.rs#L258-L316`

_private_

```
async fn compaction_integration_preserves_last_10_turns()
```

**Calls:** add_message, compact

### compaction_never_splits_a_tool_use_result_pair

*Rust Function* — `src/llm/agent/compaction.rs#L327-L400`

_private_

```
async fn compaction_never_splits_a_tool_use_result_pair()
```

**Calls:** add_message, compact

### message_has_tool_result

*Rust Function* — `src/llm/agent/compaction.rs#L126-L131`

_private_

```
fn message_has_tool_result(msg: &crate::llm::provider::types::Message) -> bool
```

**Called by:** compact

### summarise_turns

*Rust Function* — `src/llm/agent/compaction.rs#L133-L157`

_private_

```
fn summarise_turns(messages: &[crate::llm::provider::types::Message]) -> String
```

**Calls:** is_empty

**Called by:** compact, summarise_turns_truncates_multibyte_text_without_panicking

### summarise_turns_truncates_multibyte_text_without_panicking

*Rust Function* — `src/llm/agent/compaction.rs#L167-L178`

_private_

```
fn summarise_turns_truncates_multibyte_text_without_panicking()
```

**Calls:** summarise_turns

### add_message

*Rust Method* — `src/llm/agent/context.rs#L68-L73`

```
pub fn add_message(&mut self, message: Message)
```

**Calls:** estimate_message_tokens

**Called by:** compaction_fires_at_threshold, compaction_atomicity_db_failure_leaves_context_unchanged, compaction_integration_preserves_last_10_turns, compaction_never_splits_a_tool_use_result_pair, from_db_messages, test_add_message, test_would_exceed_limit, test_usage_percentage, test_trim_to_fit, send_message_with_tools_inner, prepare_message_context, build_context

### estimate_message_tokens

*Rust Method* — `src/llm/agent/context.rs#L116-L143`

_private_

```
fn estimate_message_tokens(&self, message: &Message) -> usize
```

**Calls:** estimate_tokens

**Called by:** add_message, trim_to_fit

### estimate_tokens

*Rust Method* — `src/llm/agent/context.rs#L146-L148`

_private_

```
fn estimate_tokens(text: &str) -> usize
```

**Calls:** token_count

**Called by:** with_system_prompt, estimate_message_tokens, test_token_estimation

### from_db_messages

*Rust Method* — `src/llm/agent/context.rs#L76-L102`

```
pub fn from_db_messages( session_id: Uuid, db_messages: Vec<DbMessage>, max_tokens: usize, ) -> Self
```

**Calls:** add_message

**Called by:** send_message_with_tools_inner, prepare_message_context

### inject_episodic_memories

*Rust Method* — `src/llm/agent/context.rs#L180-L188`

```
pub async fn inject_episodic_memories( &mut self, pool: &sqlx::SqlitePool, max_tokens: i32, ) -> anyhow::Result<()>
```

**Calls:** inject_into_context

**Called by:** episodic_memory_inject_3_memories_within_budget

### new

*Rust Method* — `src/llm/agent/context.rs#L48-L58`

```
pub fn new(session_id: Uuid, max_tokens: usize) -> Self
```

### should_compact

*Rust Method* — `src/llm/agent/context.rs#L159-L162`

```
pub fn should_compact(&self) -> bool
```

**Called by:** compaction_fires_at_threshold, send_message_with_tools_inner

### track_file

*Rust Method* — `src/llm/agent/context.rs#L105-L108`

```
pub fn track_file(&mut self, file: TrackedFile)
```

### trim_to_fit

*Rust Method* — `src/llm/agent/context.rs#L165-L174`

```
pub fn trim_to_fit(&mut self, required_space: usize)
```

**Calls:** would_exceed_limit, is_empty, estimate_message_tokens

**Called by:** test_trim_to_fit

### usage_percentage

*Rust Method* — `src/llm/agent/context.rs#L151-L153`

```
pub fn usage_percentage(&self) -> f64
```

**Called by:** compaction_fires_at_threshold, test_usage_percentage

### with_system_prompt

*Rust Method* — `src/llm/agent/context.rs#L61-L65`

```
pub fn with_system_prompt(mut self, prompt: String) -> Self
```

**Calls:** estimate_tokens

### would_exceed_limit

*Rust Method* — `src/llm/agent/context.rs#L111-L113`

```
pub fn would_exceed_limit(&self, additional_tokens: usize) -> bool
```

**Called by:** trim_to_fit

### test_add_message

*Rust Function* — `src/llm/agent/context.rs#L227-L236`

_private_

```
fn test_add_message()
```

**Calls:** user, add_message

### test_context_creation

*Rust Function* — `src/llm/agent/context.rs#L216-L224`

_private_

```
fn test_context_creation()
```

### test_system_prompt

*Rust Function* — `src/llm/agent/context.rs#L239-L246`

_private_

```
fn test_system_prompt()
```

### test_token_estimation

*Rust Function* — `src/llm/agent/context.rs#L249-L253`

_private_

```
fn test_token_estimation()
```

**Calls:** estimate_tokens

### test_trim_to_fit

*Rust Function* — `src/llm/agent/context.rs#L282-L298`

_private_

```
fn test_trim_to_fit()
```

**Calls:** user, add_message, len, trim_to_fit

### test_usage_percentage

*Rust Function* — `src/llm/agent/context.rs#L268-L279`

_private_

```
fn test_usage_percentage()
```

**Calls:** user, add_message, usage_percentage

### test_would_exceed_limit

*Rust Function* — `src/llm/agent/context.rs#L256-L265`

_private_

```
fn test_would_exceed_limit()
```

**Calls:** user, add_message

### token_count

*Rust Function* — `src/llm/agent/context.rs#L195-L209`

```
pub fn token_count(text: &str) -> u32
```

**Calls:** is_empty, len

**Called by:** episodic_memory_inject_3_memories_within_budget, compact, estimate_tokens, token_count_bpe_accuracy_rust_file, token_count_prose_reasonable, end_session_with_summary

### token_count_bpe_accuracy_rust_file

*Rust Function* — `src/llm/agent/context.rs#L305-L339`

_private_

```
fn token_count_bpe_accuracy_rust_file()
```

**Calls:** token_count

### token_count_empty_string

*Rust Function* — `src/llm/agent/context.rs#L342-L344`

_private_

```
fn token_count_empty_string()
```

### token_count_prose_reasonable

*Rust Function* — `src/llm/agent/context.rs#L347-L356`

_private_

```
fn token_count_prose_reasonable()
```

**Calls:** token_count

### fts_search

*Rust Method* — `src/llm/agent/memory.rs#L105-L119`

```
pub async fn fts_search(&self, query: &str) -> Result<Vec<CodebaseIndexEntry>>
```

**Called by:** fts_search_finds_symbol_by_partial_name

### index_file

*Rust Method* — `src/llm/agent/memory.rs#L61-L89`

```
pub async fn index_file(&self, path: &Path) -> Result<()>
```

**Calls:** extract_symbols, symbol_kind_str

**Called by:** start_file_watcher, index_and_query_provider_trait, index_file_twice_no_duplicate, fts_search_finds_symbol_by_partial_name, index_nonexistent_file_returns_error

### new

*Rust Method* — `src/llm/agent/memory.rs#L56-L58`

```
pub fn new(pool: SqlitePool) -> Self
```

### query_symbol

*Rust Method* — `src/llm/agent/memory.rs#L92-L102`

```
pub async fn query_symbol(&self, name: &str) -> Result<Vec<CodebaseIndexEntry>>
```

**Called by:** index_and_query_provider_trait, index_file_twice_no_duplicate

### extract_symbols

*Rust Function* — `src/llm/agent/memory.rs#L161-L196`

_private_

```
fn extract_symbols(file_path: &str, content: &str) -> Vec<CodebaseIndexEntry>
```

**Called by:** index_file

### row_to_entry

*Rust Function* — `src/llm/agent/memory.rs#L148-L158`

_private_

```
fn row_to_entry(row: (String, String, String, String, i64, i64)) -> CodebaseIndexEntry
```

**Calls:** parse, str_to_symbol_kind

### str_to_symbol_kind

*Rust Function* — `src/llm/agent/memory.rs#L136-L146`

_private_

```
fn str_to_symbol_kind(s: &str) -> SymbolKind
```

**Called by:** row_to_entry

### symbol_kind_str

*Rust Function* — `src/llm/agent/memory.rs#L124-L134`

_private_

```
fn symbol_kind_str(kind: &SymbolKind) -> &'static str
```

**Called by:** index_file

### call_provider_streaming

*Rust Method* — `src/llm/agent/service.rs#L1579-L1591`

_private_

```
async fn call_provider_streaming( provider: &Arc<dyn Provider>, request: LLMRequest, chunk_tx: Option<&mpsc::UnboundedSender<String>>, model_name: &str, ) -> crate::llm::provider::Result<LLMResponse>
```

**Calls:** with_streaming, drain_stream_to_response

**Called by:** send_message_with_tools_inner

### extract_text_from_response

*Rust Method* — `src/llm/agent/service.rs#L1661-L1678`

_private_

```
fn extract_text_from_response(response: &LLMResponse) -> String
```

**Called by:** final_text_and_thinking

### extract_thinking_from_response

*Rust Method* — `src/llm/agent/service.rs#L1720-L1735`

_private_

```
fn extract_thinking_from_response(response: &LLMResponse) -> Option<String>
```

**Calls:** is_empty

**Called by:** final_text_and_thinking

### final_text_and_thinking

*Rust Method* — `src/llm/agent/service.rs#L1698-L1717`

_private_

```
fn final_text_and_thinking(response: &LLMResponse) -> FinalText
```

**Calls:** extract_text_from_response, extract_thinking_from_response, is_empty

**Called by:** send_message, send_message_with_tools_inner, final_text_falls_back_to_thinking_when_there_is_no_visible_text, final_text_prefers_visible_text_and_keeps_thinking_separate, final_text_of_an_empty_response_is_empty

### new

*Rust Method* — `src/llm/agent/service.rs#L536-L552`

```
pub fn new(provider: Arc<dyn Provider>, context: ServiceContext) -> Self
```

### prepare_message_context

*Rust Method* — `src/llm/agent/service.rs#L1597-L1658`

_private_

```
async fn prepare_message_context( &self, session_id: Uuid, user_message: String, model: Option<String>, ) -> Result<(String, LLMRequest, MessageService, SessionService)>
```

**Calls:** get_session, list_messages_for_session, from_db_messages, system_prompt_with_env, augment_message_with_pdf, user, add_message, create_message, with_max_tokens, with_system

**Called by:** send_message, send_message_streaming

### provider_context_window

*Rust Method* — `src/llm/agent/service.rs#L653-L655`

```
pub fn provider_context_window(&self) -> Option<u32>
```

### provider_model

*Rust Method* — `src/llm/agent/service.rs#L647-L649`

```
pub fn provider_model(&self) -> &str
```

### provider_name

*Rust Method* — `src/llm/agent/service.rs#L642-L644`

```
pub fn provider_name(&self) -> &str
```

### send_message

*Rust Method* — `src/llm/agent/service.rs#L665-L736`

```
pub async fn send_message( &self, session_id: Uuid, user_message: String, model: Option<String>, ) -> Result<AgentResponse>
```

**Calls:** prepare_message_context, final_text_and_thinking, create_message, update_message_usage, update_message_metrics, update_session_usage

### send_message_streaming

*Rust Method* — `src/llm/agent/service.rs#L741-L768`

```
pub async fn send_message_streaming( &self, session_id: Uuid, user_message: String, model: Option<String>, ) -> Result<AgentStreamResponse>
```

**Calls:** prepare_message_context, with_streaming

### send_message_with_tools

*Rust Method* — `src/llm/agent/service.rs#L777-L785`

```
pub async fn send_message_with_tools( &self, session_id: Uuid, user_message: String, model: Option<String>, ) -> Result<AgentResponse>
```

**Calls:** send_message_with_tools_and_mode

**Called by:** cmd_run, launch, test_send_message_with_tool_execution

### send_message_with_tools_and_mode

*Rust Method* — `src/llm/agent/service.rs#L788-L797`

```
pub async fn send_message_with_tools_and_mode( &self, session_id: Uuid, user_message: String, model: Option<String>, read_only_mode: bool, ) -> Result<AgentResponse>
```

**Calls:** send_message_with_tools_inner

**Called by:** send_message_with_tools

### send_message_with_tools_and_mode_streaming

*Rust Method* — `src/llm/agent/service.rs#L801-L817`

```
pub async fn send_message_with_tools_and_mode_streaming( &self, session_id: Uuid, user_message: String, model: Option<String>, read_only_mode: bool, chunk_tx: mpsc::UnboundedSender<String>, ) -> Result<AgentResponse>
```

**Calls:** send_message_with_tools_inner

**Called by:** send_message

### send_message_with_tools_inner

*Rust Method* — `src/llm/agent/service.rs#L820-L1573`

_private_

```
async fn send_message_with_tools_inner( &self, session_id: Uuid, user_message: String, model: Option<String>, read_only_mode: bool, chunk_tx: Option<mpsc::UnboundedSender<String>>, ) -> Result<AgentResponse>
```

**Calls:** get_session, list_messages_for_session, from_db_messages, system_prompt_with_env, augment_message_with_pdf, user, add_message, create_message, classify_tier, with_auto_approve, with_read_only_mode, with_file_read_cache, with_sub_agent_launcher, with_max_tokens, with_system, get_tool_definitions, with_tools, call_provider_streaming, is_empty, tool_call_signature, len, has_mutating_capability, is_parallelizable, from_tool, insert_for_tool, plan_completion_rejection, is_trusted, invalidate_matching, should_compact, compact, final_text_and_thinking, update_message_usage, update_message_metrics, update_session_usage

**Called by:** send_message_with_tools_and_mode, send_message_with_tools_and_mode_streaming

### set_provider

*Rust Method* — `src/llm/agent/service.rs#L637-L639`

```
pub fn set_provider(&mut self, provider: Arc<dyn Provider>)
```

**Called by:** switch_provider_to_ollama_model

### system_prompt_with_env

*Rust Method* — `src/llm/agent/service.rs#L614-L623`

_private_

```
fn system_prompt_with_env(&self) -> Option<String>
```

**Called by:** send_message_with_tools_inner, prepare_message_context, system_prompt_tells_the_model_the_working_directory

### with_allow_sub_agents

*Rust Method* — `src/llm/agent/service.rs#L627-L630`

```
pub fn with_allow_sub_agents(mut self, allow: bool) -> Self
```

**Called by:** launch

### with_approval_callback

*Rust Method* — `src/llm/agent/service.rs#L591-L594`

```
pub fn with_approval_callback(mut self, callback: Option<ApprovalCallback>) -> Self
```

**Called by:** cmd_chat

### with_auto_approve_tools

*Rust Method* — `src/llm/agent/service.rs#L585-L588`

```
pub fn with_auto_approve_tools(mut self, auto_approve: bool) -> Self
```

**Called by:** cmd_run, test_send_message_with_tool_execution

### with_max_tool_iterations

*Rust Method* — `src/llm/agent/service.rs#L573-L576`

```
pub fn with_max_tool_iterations(mut self, max: usize) -> Self
```

**Called by:** cmd_chat, cmd_run, launch

### with_model_router

*Rust Method* — `src/llm/agent/service.rs#L555-L558`

```
pub fn with_model_router(mut self, router: ModelRouter) -> Self
```

### with_pool

*Rust Method* — `src/llm/agent/service.rs#L561-L564`

```
pub fn with_pool(mut self, pool: Arc<sqlx::SqlitePool>) -> Self
```

### with_system_prompt

*Rust Method* — `src/llm/agent/service.rs#L567-L570`

```
pub fn with_system_prompt(mut self, prompt: String) -> Self
```

### with_tool_registry

*Rust Method* — `src/llm/agent/service.rs#L579-L582`

```
pub fn with_tool_registry(mut self, registry: Arc<ToolRegistry>) -> Self
```

**Called by:** cmd_chat, cmd_run, launch, test_send_message_with_tool_execution, create_error_agent, create_test_agent

### with_working_directory

*Rust Method* — `src/llm/agent/service.rs#L597-L600`

```
pub fn with_working_directory(mut self, working_directory: std::path::PathBuf) -> Self
```

### launch

*Rust Method* — `src/llm/agent/service.rs#L1827-L1875`

_private_

```
async fn launch( &self, _agent_id: uuid::Uuid, description: &str, prompt: &str, ) -> std::result::Result<(), String>
```

**Calls:** with_tool_registry, with_max_tool_iterations, with_allow_sub_agents, send_message_with_tools

**Called by:** sub_agent_launcher_does_not_auto_approve_tools, execute

### new

*Rust Method* — `src/llm/agent/service.rs#L1808-L1822`

```
pub fn new( provider: Arc<dyn Provider>, context: ServiceContext, tool_registry: Arc<ToolRegistry>, working_directory: std::path::PathBuf, system_prompt: Option<String>, ) -> Self
```

### fmt

*Rust Method* — `src/llm/agent/service.rs#L1800-L1804`

_private_

```
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

### calculate_cost

*Rust Method* — `src/llm/agent/service.rs#L1933-L1935`

_private_

```
fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64
```

### complete

*Rust Method* — `src/llm/agent/service.rs#L1890-L1908`

_private_

```
async fn complete( &self, _request: LLMRequest, ) -> crate::llm::provider::Result<LLMResponse>
```

### context_window

*Rust Method* — `src/llm/agent/service.rs#L1929-L1931`

_private_

```
fn context_window(&self, _model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/llm/agent/service.rs#L1921-L1923`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/llm/agent/service.rs#L1917-L1919`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/llm/agent/service.rs#L1910-L1915`

_private_

```
async fn stream( &self, _request: LLMRequest, ) -> crate::llm::provider::Result<ProviderStream>
```

### supported_models

*Rust Method* — `src/llm/agent/service.rs#L1925-L1927`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### new

*Rust Method* — `src/llm/agent/service.rs#L2272-L2276`

_private_

```
fn new() -> Self
```

### calculate_cost

*Rust Method* — `src/llm/agent/service.rs#L2354-L2356`

_private_

```
fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64
```

### complete

*Rust Method* — `src/llm/agent/service.rs#L2281-L2329`

_private_

```
async fn complete( &self, _request: LLMRequest, ) -> crate::llm::provider::Result<LLMResponse>
```

### context_window

*Rust Method* — `src/llm/agent/service.rs#L2350-L2352`

_private_

```
fn context_window(&self, _model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/llm/agent/service.rs#L2342-L2344`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/llm/agent/service.rs#L2338-L2340`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/llm/agent/service.rs#L2331-L2336`

_private_

```
async fn stream( &self, _request: LLMRequest, ) -> crate::llm::provider::Result<ProviderStream>
```

### supported_models

*Rust Method* — `src/llm/agent/service.rs#L2346-L2348`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### capabilities

*Rust Method* — `src/llm/agent/service.rs#L2381-L2383`

_private_

```
fn capabilities(&self) -> Vec<crate::llm::tools::ToolCapability>
```

### description

*Rust Method* — `src/llm/agent/service.rs#L2368-L2370`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/agent/service.rs#L2389-L2397`

_private_

```
async fn execute( &self, _input: serde_json::Value, _context: &crate::llm::tools::ToolExecutionContext, ) -> crate::llm::tools::Result<crate::llm::tools::ToolResult>
```

### input_schema

*Rust Method* — `src/llm/agent/service.rs#L2372-L2379`

_private_

```
fn input_schema(&self) -> serde_json::Value
```

### name

*Rust Method* — `src/llm/agent/service.rs#L2364-L2366`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/agent/service.rs#L2385-L2387`

_private_

```
fn requires_approval(&self) -> bool
```

### apply_streamed_tool_input

*Rust Function* — `src/llm/agent/service.rs#L373-L398`

_private_

```
fn apply_streamed_tool_input(block: ContentBlock, json_buf: &str) -> ContentBlock
```

**Calls:** is_empty, from_str

**Called by:** drain_stream_to_response

### create_test_service

*Rust Function* — `src/llm/agent/service.rs#L2149-L2167`

_private_

```
async fn create_test_service() -> (AgentService, Uuid)
```

**Calls:** run_migrations

### drain_stream_assembles_anthropic_tool_input_from_json_deltas

*Rust Function* — `src/llm/agent/service.rs#L2561-L2624`

_private_

```
async fn drain_stream_assembles_anthropic_tool_input_from_json_deltas()
```

**Calls:** drain_stream_to_response

### drain_stream_to_response

*Rust Function* — `src/llm/agent/service.rs#L406-L532`

_private_

```
async fn drain_stream_to_response( stream: ProviderStream, chunk_tx: Option<&mpsc::UnboundedSender<String>>, model_name: &str, ) -> crate::llm::provider::Result<LLMResponse>
```

**Calls:** next, apply_streamed_tool_input, is_empty, route_text_delta, extract_think_tags

**Called by:** call_provider_streaming, streamed_ollama_tool_call_survives_drain, drain_stream_to_response_carries_perf_metrics_through, drain_stream_assembles_anthropic_tool_input_from_json_deltas

### drain_stream_to_response_carries_perf_metrics_through

*Rust Function* — `src/llm/agent/service.rs#L2506-L2558`

_private_

```
async fn drain_stream_to_response_carries_perf_metrics_through()
```

**Calls:** drain_stream_to_response

### final_text_falls_back_to_thinking_when_there_is_no_visible_text

*Rust Function* — `src/llm/agent/service.rs#L2085-L2115`

_private_

```
fn final_text_falls_back_to_thinking_when_there_is_no_visible_text()
```

**Calls:** response_with, final_text_and_thinking

### final_text_of_an_empty_response_is_empty

*Rust Function* — `src/llm/agent/service.rs#L2141-L2147`

_private_

```
fn final_text_of_an_empty_response_is_empty()
```

**Calls:** response_with, final_text_and_thinking

### final_text_prefers_visible_text_and_keeps_thinking_separate

*Rust Function* — `src/llm/agent/service.rs#L2120-L2136`

_private_

```
fn final_text_prefers_visible_text_and_keeps_thinking_separate()
```

**Calls:** response_with, final_text_and_thinking

### has_mutating_capability

*Rust Function* — `src/llm/agent/service.rs#L48-L57`

_private_

```
fn has_mutating_capability(caps: &[ToolCapability]) -> bool
```

**Called by:** send_message_with_tools_inner

### is_parallelizable

*Rust Function* — `src/llm/agent/service.rs#L227-L240`

```
pub fn is_parallelizable(tool_name: &str) -> bool
```

**Called by:** send_message_with_tools_inner

### loop_detection_recovery_message_logic

*Rust Function* — `src/llm/agent/service.rs#L2439-L2446`

_private_

```
fn loop_detection_recovery_message_logic()
```

### plan_completion_gate_decision_matrix

*Rust Function* — `src/llm/agent/service.rs#L1991-L2056`

_private_

```
fn plan_completion_gate_decision_matrix()
```

**Calls:** plan_completion_rejection

### plan_completion_rejection

*Rust Function* — `src/llm/agent/service.rs#L157-L224`

_private_

```
fn plan_completion_rejection( input: &Value, mutating_evidence: usize, working_directory: &std::path::Path, session_id: Uuid, ) -> Option<String>
```

**Calls:** is_empty

**Called by:** send_message_with_tools_inner, plan_completion_gate_decision_matrix

### response_with

*Rust Function* — `src/llm/agent/service.rs#L2058-L2071`

_private_

```
fn response_with(content: Vec<ContentBlock>) -> LLMResponse
```

**Called by:** final_text_falls_back_to_thinking_when_there_is_no_visible_text, final_text_prefers_visible_text_and_keeps_thinking_separate, final_text_of_an_empty_response_is_empty

### route_text_delta

*Rust Function* — `src/llm/agent/service.rs#L318-L363`

_private_

```
fn route_text_delta( input: &str, in_think: &mut bool, text_buf: &mut String, thinking_buf: &mut String, chunk_tx: Option<&mpsc::UnboundedSender<String>>, )
```

**Calls:** len, is_empty

**Called by:** drain_stream_to_response

### signature_accepts_file_path_alias

*Rust Function* — `src/llm/agent/service.rs#L1959-L1966`

_private_

```
fn signature_accepts_file_path_alias()
```

**Calls:** tool_call_signature

### signature_distinguishes_same_tool_different_args

*Rust Function* — `src/llm/agent/service.rs#L1969-L1984`

_private_

```
fn signature_distinguishes_same_tool_different_args()
```

### signature_uses_path_key_so_different_edits_do_not_collide

*Rust Function* — `src/llm/agent/service.rs#L1945-L1956`

_private_

```
fn signature_uses_path_key_so_different_edits_do_not_collide()
```

**Calls:** tool_call_signature

### streamed_ollama_tool_call_survives_drain

*Rust Function* — `src/llm/agent/service.rs#L2458-L2503`

_private_

```
async fn streamed_ollama_tool_call_survives_drain()
```

**Calls:** default_local, with_tools, with_max_tokens, with_system, with_streaming, drain_stream_to_response

### sub_agent_launcher_does_not_auto_approve_tools

*Rust Function* — `src/llm/agent/service.rs#L2247-L2264`

_private_

```
async fn sub_agent_launcher_does_not_auto_approve_tools()
```

**Calls:** run_migrations, launch

### system_prompt_tells_the_model_the_working_directory

*Rust Function* — `src/llm/agent/service.rs#L2210-L2230`

_private_

```
async fn system_prompt_tells_the_model_the_working_directory()
```

**Calls:** system_prompt_with_env

### system_prompt_with_env_is_none_when_no_prompt_is_set

*Rust Function* — `src/llm/agent/service.rs#L2235-L2238`

_private_

```
async fn system_prompt_with_env_is_none_when_no_prompt_is_set()
```

### test_agent_service_creation

*Rust Function* — `src/llm/agent/service.rs#L2170-L2173`

_private_

```
async fn test_agent_service_creation()
```

### test_send_message

*Rust Function* — `src/llm/agent/service.rs#L2176-L2187`

_private_

```
async fn test_send_message()
```

### test_send_message_with_system_prompt

*Rust Function* — `src/llm/agent/service.rs#L2190-L2202`

_private_

```
async fn test_send_message_with_system_prompt()
```

### test_send_message_with_tool_execution

*Rust Function* — `src/llm/agent/service.rs#L2401-L2436`

_private_

```
async fn test_send_message_with_tool_execution()
```

**Calls:** run_migrations, register, with_tool_registry, with_auto_approve_tools, send_message_with_tools

### tool_call_signature

*Rust Function* — `src/llm/agent/service.rs#L71-L140`

_private_

```
fn tool_call_signature(name: &str, input: &Value) -> String
```

**Called by:** send_message_with_tools_inner, signature_uses_path_key_so_different_edits_do_not_collide, signature_accepts_file_path_alias

### augment_message_with_pdf

*Rust Function* — `src/llm/pdf_context.rs#L78-L127`

```
pub async fn augment_message_with_pdf(message: &str, cwd: &Path) -> String
```

**Calls:** looks_like_pdf_path, extract_pdf_text, is_empty, truncate_at_char_boundary, len

**Called by:** send_message_with_tools_inner, prepare_message_context, augment_returns_original_when_no_pdf, augment_returns_original_on_extraction_failure

### augment_returns_original_on_extraction_failure

*Rust Function* — `src/llm/pdf_context.rs#L202-L214`

_private_

```
async fn augment_returns_original_on_extraction_failure()
```

**Calls:** augment_message_with_pdf

### augment_returns_original_when_no_pdf

*Rust Function* — `src/llm/pdf_context.rs#L195-L199`

_private_

```
async fn augment_returns_original_when_no_pdf()
```

**Calls:** augment_message_with_pdf

### case_insensitive_extension

*Rust Function* — `src/llm/pdf_context.rs#L160-L168`

_private_

```
fn case_insensitive_extension()
```

### detects_absolute_pdf_token

*Rust Function* — `src/llm/pdf_context.rs#L136-L145`

_private_

```
fn detects_absolute_pdf_token()
```

**Calls:** looks_like_pdf_path

### detects_relative_pdf_token

*Rust Function* — `src/llm/pdf_context.rs#L148-L157`

_private_

```
fn detects_relative_pdf_token()
```

**Calls:** looks_like_pdf_path

### extract_pdf_text

*Rust Function* — `src/llm/pdf_context.rs#L55-L60`

```
pub fn extract_pdf_text(path: &Path) -> Result<String, String>
```

**Called by:** augment_message_with_pdf

### looks_like_pdf_path

*Rust Function* — `src/llm/pdf_context.rs#L28-L49`

```
pub fn looks_like_pdf_path(text: &str, cwd: &Path) -> Option<PathBuf>
```

**Called by:** augment_message_with_pdf, detects_absolute_pdf_token, detects_relative_pdf_token, returns_none_for_missing_file, returns_none_when_no_pdf

### returns_none_for_missing_file

*Rust Function* — `src/llm/pdf_context.rs#L182-L186`

_private_

```
fn returns_none_for_missing_file()
```

**Calls:** looks_like_pdf_path

### returns_none_when_no_pdf

*Rust Function* — `src/llm/pdf_context.rs#L189-L192`

_private_

```
fn returns_none_when_no_pdf()
```

**Calls:** looks_like_pdf_path

### strips_surrounding_quotes

*Rust Function* — `src/llm/pdf_context.rs#L171-L179`

_private_

```
fn strips_surrounding_quotes()
```

### from_anthropic_response

*Rust Method* — `src/llm/provider/anthropic.rs#L102-L126`

_private_

```
fn from_anthropic_response(&self, response: AnthropicResponse) -> LLMResponse
```

**Called by:** complete

### handle_error

*Rust Method* — `src/llm/provider/anthropic.rs#L129-L186`

_private_

```
async fn handle_error(&self, response: reqwest::Response) -> ProviderError
```

### headers

*Rust Method* — `src/llm/provider/anthropic.rs#L61-L83`

_private_

```
fn headers(&self) -> Result<reqwest::header::HeaderMap>
```

**Calls:** parse

### new

*Rust Method* — `src/llm/provider/anthropic.rs#L35-L45`

```
pub fn new(api_key: String) -> Self
```

### calculate_cost

*Rust Method* — `src/llm/provider/anthropic.rs#L328-L361`

_private_

```
fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64
```

### complete

*Rust Method* — `src/llm/provider/anthropic.rs#L191-L246`

_private_

```
async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>
```

**Calls:** len, to_anthropic_request, retry_with_backoff, is_success, from_anthropic_response

### context_window

*Rust Method* — `src/llm/provider/anthropic.rs#L318-L326`

_private_

```
fn context_window(&self, model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/llm/provider/anthropic.rs#L305-L307`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/llm/provider/anthropic.rs#L301-L303`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/llm/provider/anthropic.rs#L248-L287`

_private_

```
async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>
```

**Calls:** len, to_anthropic_request, retry_with_backoff, is_success, parse_anthropic_sse_stream

### supported_models

*Rust Method* — `src/llm/provider/anthropic.rs#L309-L316`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### supports_streaming

*Rust Method* — `src/llm/provider/anthropic.rs#L289-L291`

_private_

```
fn supports_streaming(&self) -> bool
```

### supports_tools

*Rust Method* — `src/llm/provider/anthropic.rs#L293-L295`

_private_

```
fn supports_tools(&self) -> bool
```

### supports_vision

*Rust Method* — `src/llm/provider/anthropic.rs#L297-L299`

_private_

```
fn supports_vision(&self) -> bool
```

### to_anthropic_request

*Rust Method* — `src/llm/provider/anthropic.rs#L86-L98`

_private_

```
fn to_anthropic_request(&self, request: LLMRequest) -> AnthropicRequest
```

**Called by:** complete, stream

### with_client

*Rust Method* — `src/llm/provider/anthropic.rs#L48-L50`

```
pub fn with_client(api_key: String, client: Client) -> Self
```

### parse_anthropic_sse_stream

*Rust Function* — `src/llm/provider/anthropic.rs#L376-L427`

_private_

```
fn parse_anthropic_sse_stream( byte_stream: impl futures::Stream<Item = reqwest::Result<bytes::Bytes>> + Send + 'static, ) -> impl futures::Stream<Item = Result<StreamEvent>> + Send + 'static
```

**Calls:** is_empty

**Called by:** stream, sse_stream_yields_every_event_in_a_single_chunk, sse_stream_reassembles_an_event_split_across_chunks

### sse_stream_reassembles_an_event_split_across_chunks

*Rust Function* — `src/llm/provider/anthropic.rs#L591-L611`

_private_

```
async fn sse_stream_reassembles_an_event_split_across_chunks()
```

**Calls:** parse_anthropic_sse_stream

### sse_stream_yields_every_event_in_a_single_chunk

*Rust Function* — `src/llm/provider/anthropic.rs#L562-L584`

_private_

```
async fn sse_stream_yields_every_event_in_a_single_chunk()
```

**Calls:** parse_anthropic_sse_stream

### test_anthropic_provider_creation

*Rust Function* — `src/llm/provider/anthropic.rs#L489-L493`

_private_

```
fn test_anthropic_provider_creation()
```

### test_capabilities

*Rust Function* — `src/llm/provider/anthropic.rs#L614-L619`

_private_

```
fn test_capabilities()
```

### test_context_window

*Rust Function* — `src/llm/provider/anthropic.rs#L504-L511`

_private_

```
fn test_context_window()
```

### test_cost_calculation

*Rust Function* — `src/llm/provider/anthropic.rs#L514-L524`

_private_

```
fn test_cost_calculation()
```

### test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids

*Rust Function* — `src/llm/provider/anthropic.rs#L527-L546`

_private_

```
fn test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids()
```

### test_cost_calculation_unknown_model_family_returns_zero

*Rust Function* — `src/llm/provider/anthropic.rs#L549-L555`

_private_

```
fn test_cost_calculation_unknown_model_family_returns_zero()
```

### test_supported_models

*Rust Function* — `src/llm/provider/anthropic.rs#L496-L501`

_private_

```
fn test_supported_models()
```

### new

*Rust Method* — `src/llm/provider/azure.rs#L39-L57`

```
pub fn new(api_key: String, resource_name: String, deployment_id: String) -> Self
```

**Calls:** with_api_key_header

### calculate_cost

*Rust Method* — `src/llm/provider/azure.rs#L121-L136`

_private_

```
fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64
```

### complete

*Rust Method* — `src/llm/provider/azure.rs#L95-L97`

_private_

```
async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>
```

### context_window

*Rust Method* — `src/llm/provider/azure.rs#L113-L119`

_private_

```
fn context_window(&self, model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/llm/provider/azure.rs#L91-L93`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/llm/provider/azure.rs#L87-L89`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/llm/provider/azure.rs#L99-L101`

_private_

```
async fn stream(&self, request: LLMRequest) -> Result<super::ProviderStream>
```

### supported_models

*Rust Method* — `src/llm/provider/azure.rs#L103-L111`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### with_default_model

*Rust Method* — `src/llm/provider/azure.rs#L79-L82`

```
pub fn with_default_model(mut self, model: String) -> Self
```

### with_endpoint

*Rust Method* — `src/llm/provider/azure.rs#L68-L76`

```
pub fn with_endpoint(api_key: String, endpoint: String) -> Self
```

**Calls:** with_api_key_header

**Called by:** try_create_azure

### test_azure_context_window

*Rust Function* — `src/llm/provider/azure.rs#L157-L167`

_private_

```
fn test_azure_context_window()
```

### test_azure_cost_calculation

*Rust Function* — `src/llm/provider/azure.rs#L170-L182`

_private_

```
fn test_azure_cost_calculation()
```

### test_azure_provider_creation

*Rust Function* — `src/llm/provider/azure.rs#L144-L154`

_private_

```
fn test_azure_provider_creation()
```

### test_azure_supported_models

*Rust Function* — `src/llm/provider/azure.rs#L185-L195`

_private_

```
fn test_azure_supported_models()
```

### is_retryable

*Rust Method* — `src/llm/provider/error.rs#L67-L75`

```
pub fn is_retryable(&self) -> bool
```

### status_code

*Rust Method* — `src/llm/provider/error.rs#L78-L83`

```
pub fn status_code(&self) -> Option<u16>
```

### test_error_retryable

*Rust Function* — `src/llm/provider/error.rs#L94-L114`

_private_

```
fn test_error_retryable()
```

### test_status_code

*Rust Function* — `src/llm/provider/error.rs#L117-L127`

_private_

```
fn test_status_code()
```

### is_failover_error

*Rust Method* — `src/llm/provider/factory.rs#L36-L43`

_private_

```
fn is_failover_error(err: &ProviderError) -> bool
```

**Called by:** complete, stream

### new

*Rust Method* — `src/llm/provider/factory.rs#L31-L34`

```
pub fn new(chain: Vec<Arc<dyn Provider>>) -> Self
```

### calculate_cost

*Rust Method* — `src/llm/provider/factory.rs#L113-L118`

_private_

```
fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64
```

### complete

*Rust Method* — `src/llm/provider/factory.rs#L48-L69`

_private_

```
async fn complete( &self, request: super::types::LLMRequest, ) -> super::error::Result<super::types::LLMResponse>
```

**Calls:** is_failover_error

### context_window

*Rust Method* — `src/llm/provider/factory.rs#L109-L111`

_private_

```
fn context_window(&self, model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/llm/provider/factory.rs#L98-L100`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/llm/provider/factory.rs#L94-L96`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/llm/provider/factory.rs#L71-L92`

_private_

```
async fn stream( &self, request: super::types::LLMRequest, ) -> super::error::Result<super::r#trait::ProviderStream>
```

**Calls:** is_failover_error

### supported_models

*Rust Method* — `src/llm/provider/factory.rs#L102-L107`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### calculate_cost

*Rust Method* — `src/llm/provider/factory.rs#L541-L543`

_private_

```
fn calculate_cost(&self, _model: &str, _in: u32, _out: u32) -> f64
```

### complete

*Rust Method* — `src/llm/provider/factory.rs#L523-L525`

_private_

```
async fn complete(&self, _req: LLMRequest) -> ProviderResult<LLMResponse>
```

### context_window

*Rust Method* — `src/llm/provider/factory.rs#L538-L540`

_private_

```
fn context_window(&self, _model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/llm/provider/factory.rs#L532-L534`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/llm/provider/factory.rs#L529-L531`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/llm/provider/factory.rs#L526-L528`

_private_

```
async fn stream(&self, _req: LLMRequest) -> ProviderResult<ProviderStream>
```

### supported_models

*Rust Method* — `src/llm/provider/factory.rs#L535-L537`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### calculate_cost

*Rust Method* — `src/llm/provider/factory.rs#L587-L589`

_private_

```
fn calculate_cost(&self, _model: &str, _in: u32, _out: u32) -> f64
```

### complete

*Rust Method* — `src/llm/provider/factory.rs#L553-L569`

_private_

```
async fn complete(&self, _req: LLMRequest) -> ProviderResult<LLMResponse>
```

### context_window

*Rust Method* — `src/llm/provider/factory.rs#L584-L586`

_private_

```
fn context_window(&self, _model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/llm/provider/factory.rs#L578-L580`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/llm/provider/factory.rs#L575-L577`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/llm/provider/factory.rs#L570-L574`

_private_

```
async fn stream(&self, _req: LLMRequest) -> ProviderResult<ProviderStream>
```

### supported_models

*Rust Method* — `src/llm/provider/factory.rs#L581-L583`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### configure_openai

*Rust Function* — `src/llm/provider/factory.rs#L466-L473`

_private_

```
fn configure_openai(mut provider: OpenAIProvider, config: &ProviderConfig) -> OpenAIProvider
```

**Called by:** try_create_openai

### configure_qwen

*Rust Function* — `src/llm/provider/factory.rs#L380-L435`

_private_

```
fn configure_qwen(mut provider: QwenProvider, config: &QwenProviderConfig) -> QwenProvider
```

**Calls:** with_tool_parser, with_thinking_budget

**Called by:** try_create_qwen, configure_qwen_auto_selects_openai_parser_for_coder_next, configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection, configure_qwen_keeps_hermes_default_for_other_models

### configure_qwen_auto_selects_openai_parser_for_coder_next

*Rust Function* — `src/llm/provider/factory.rs#L968-L989`

_private_

```
fn configure_qwen_auto_selects_openai_parser_for_coder_next()
```

**Calls:** configure_qwen

### configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection

*Rust Function* — `src/llm/provider/factory.rs#L994-L1015`

_private_

```
fn configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection()
```

**Calls:** configure_qwen

### configure_qwen_keeps_hermes_default_for_other_models

*Rust Function* — `src/llm/provider/factory.rs#L1020-L1041`

_private_

```
fn configure_qwen_keeps_hermes_default_for_other_models()
```

**Calls:** configure_qwen

### create_anthropic

*Rust Function* — `src/llm/provider/factory.rs#L476-L502`

_private_

```
fn create_anthropic(config: &Config) -> Result<Arc<dyn Provider>>
```

**Called by:** create_provider

### create_provider

*Rust Function* — `src/llm/provider/factory.rs#L141-L169`

```
pub fn create_provider(config: &Config) -> Result<Arc<dyn Provider>>
```

**Calls:** try_create_qwen, try_create_openai, try_create_gemini, try_create_azure, create_anthropic

**Called by:** cmd_chat, cmd_run, test_create_provider_with_anthropic, test_create_provider_with_openai, test_create_provider_with_azure, test_disabled_azure_falls_through_to_anthropic, test_create_provider_with_gemini, test_create_provider_with_gemini_custom_base_url_and_model, gemini_without_api_key_falls_through_to_anthropic, disabled_gemini_is_skipped_in_favour_of_the_next_provider, disabled_openai_is_skipped_in_favour_of_the_next_provider, disabled_qwen_is_skipped_in_favour_of_the_next_provider, disabled_anthropic_fallback_fails_with_a_clear_message, test_create_provider_with_qwen, test_create_provider_no_credentials

### disabled_anthropic_fallback_fails_with_a_clear_message

*Rust Function* — `src/llm/provider/factory.rs#L909-L932`

_private_

```
fn disabled_anthropic_fallback_fails_with_a_clear_message()
```

**Calls:** create_provider

### disabled_gemini_is_skipped_in_favour_of_the_next_provider

*Rust Function* — `src/llm/provider/factory.rs#L815-L841`

_private_

```
fn disabled_gemini_is_skipped_in_favour_of_the_next_provider()
```

**Calls:** create_provider

### disabled_openai_is_skipped_in_favour_of_the_next_provider

*Rust Function* — `src/llm/provider/factory.rs#L849-L875`

_private_

```
fn disabled_openai_is_skipped_in_favour_of_the_next_provider()
```

**Calls:** create_provider

### disabled_qwen_is_skipped_in_favour_of_the_next_provider

*Rust Function* — `src/llm/provider/factory.rs#L878-L903`

_private_

```
fn disabled_qwen_is_skipped_in_favour_of_the_next_provider()
```

**Calls:** create_provider

### gemini_without_api_key_falls_through_to_anthropic

*Rust Function* — `src/llm/provider/factory.rs#L786-L812`

_private_

```
fn gemini_without_api_key_falls_through_to_anthropic()
```

**Calls:** create_provider

### ollama_provider_from_config

*Rust Function* — `src/llm/provider/factory.rs#L272-L323`

```
pub fn ollama_provider_from_config( cfg: &crate::config::OllamaProviderConfig, model_override: Option<&str>, ) -> super::ollama::OllamaProvider
```

**Calls:** with_keep_alive, with_num_ctx, with_think, is_empty, from_config, with_per_model

**Called by:** try_create_ollama, build_ollama_provider

### test_create_provider_no_credentials

*Rust Function* — `src/llm/provider/factory.rs#L1044-L1060`

_private_

```
fn test_create_provider_no_credentials()
```

**Calls:** create_provider

### test_create_provider_with_anthropic

*Rust Function* — `src/llm/provider/factory.rs#L629-L647`

_private_

```
fn test_create_provider_with_anthropic()
```

**Calls:** create_provider

### test_create_provider_with_azure

*Rust Function* — `src/llm/provider/factory.rs#L682-L709`

_private_

```
fn test_create_provider_with_azure()
```

**Calls:** create_provider

### test_create_provider_with_gemini

*Rust Function* — `src/llm/provider/factory.rs#L740-L763`

_private_

```
fn test_create_provider_with_gemini()
```

**Calls:** create_provider

### test_create_provider_with_gemini_custom_base_url_and_model

*Rust Function* — `src/llm/provider/factory.rs#L766-L783`

_private_

```
fn test_create_provider_with_gemini_custom_base_url_and_model()
```

**Calls:** create_provider

### test_create_provider_with_openai

*Rust Function* — `src/llm/provider/factory.rs#L650-L674`

_private_

```
fn test_create_provider_with_openai()
```

**Calls:** create_provider

### test_create_provider_with_qwen

*Rust Function* — `src/llm/provider/factory.rs#L935-L960`

_private_

```
fn test_create_provider_with_qwen()
```

**Calls:** create_provider

### test_disabled_azure_falls_through_to_anthropic

*Rust Function* — `src/llm/provider/factory.rs#L714-L737`

_private_

```
fn test_disabled_azure_falls_through_to_anthropic()
```

**Calls:** create_provider

### test_failover_all_fail_returns_last_error

*Rust Function* — `src/llm/provider/factory.rs#L614-L626`

_private_

```
async fn test_failover_all_fail_returns_last_error()
```

### test_failover_on_rate_limit_tries_next_provider

*Rust Function* — `src/llm/provider/factory.rs#L593-L611`

_private_

```
async fn test_failover_on_rate_limit_tries_next_provider()
```

### try_create_azure

*Rust Function* — `src/llm/provider/factory.rs#L178-L204`

_private_

```
fn try_create_azure(config: &Config) -> Result<Option<Arc<dyn Provider>>>
```

**Calls:** with_endpoint

**Called by:** create_provider

### try_create_gemini

*Rust Function* — `src/llm/provider/factory.rs#L211-L237`

_private_

```
fn try_create_gemini(config: &Config) -> Result<Option<Arc<dyn Provider>>>
```

**Called by:** create_provider

### try_create_ollama

*Rust Function* — `src/llm/provider/factory.rs#L241-L258`

_private_

```
fn try_create_ollama(config: &Config) -> Result<Option<Arc<dyn Provider>>>
```

**Calls:** ollama_provider_from_config

### try_create_ollama

*Rust Function* — `src/llm/provider/factory.rs#L329-L337`

_private_

```
fn try_create_ollama(config: &Config) -> Result<Option<Arc<dyn Provider>>>
```

### try_create_openai

*Rust Function* — `src/llm/provider/factory.rs#L438-L463`

_private_

```
fn try_create_openai(config: &Config) -> Result<Option<Arc<dyn Provider>>>
```

**Calls:** configure_openai

**Called by:** create_provider

### try_create_qwen

*Rust Function* — `src/llm/provider/factory.rs#L340-L377`

_private_

```
fn try_create_qwen(config: &Config) -> Result<Option<Arc<dyn Provider>>>
```

**Calls:** configure_qwen, dashscope_cn, dashscope_intl

**Called by:** create_provider

### text

*Rust Method* — `src/llm/provider/gemini.rs#L759-L764`

_private_

```
fn text(text: String) -> Self
```

**Called by:** to_gemini_request, execute

### with_inline_data

*Rust Method* — `src/llm/provider/gemini.rs#L766-L772`

_private_

```
fn with_inline_data(mut self, mime_type: String, data: String) -> Self
```

### from_gemini_response

*Rust Method* — `src/llm/provider/gemini.rs#L265-L337`

_private_

```
fn from_gemini_response(&self, response: GeminiResponse, model: &str) -> LLMResponse
```

**Calls:** next, is_empty

**Called by:** complete, test_from_gemini_response_maps_tool_use, test_from_gemini_response_maps_thinking

### generate_url

*Rust Method* — `src/llm/provider/gemini.rs#L101-L103`

_private_

```
fn generate_url(&self, model: &str) -> String
```

**Called by:** complete

### handle_error

*Rust Method* — `src/llm/provider/gemini.rs#L339-L348`

_private_

```
async fn handle_error(&self, response: reqwest::Response) -> ProviderError
```

**Calls:** build_gemini_error

### headers

*Rust Method* — `src/llm/provider/gemini.rs#L83-L99`

_private_

```
fn headers(&self) -> Result<reqwest::header::HeaderMap>
```

**Calls:** parse

### new

*Rust Method* — `src/llm/provider/gemini.rs#L47-L62`

```
pub fn new(api_key: String) -> Self
```

### calculate_cost

*Rust Method* — `src/llm/provider/gemini.rs#L698-L715`

_private_

```
fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64
```

### complete

*Rust Method* — `src/llm/provider/gemini.rs#L542-L591`

_private_

```
async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>
```

**Calls:** to_gemini_request, generate_url, retry_with_backoff, is_success, from_gemini_response

### context_window

*Rust Method* — `src/llm/provider/gemini.rs#L683-L696`

_private_

```
fn context_window(&self, model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/llm/provider/gemini.rs#L660-L664`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/llm/provider/gemini.rs#L656-L658`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/llm/provider/gemini.rs#L593-L641`

_private_

```
async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>
```

**Calls:** to_gemini_request, stream_url, retry_with_backoff, is_success, next, parse_gemini_sse

### supported_models

*Rust Method* — `src/llm/provider/gemini.rs#L666-L681`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### supports_streaming

*Rust Method* — `src/llm/provider/gemini.rs#L643-L645`

_private_

```
fn supports_streaming(&self) -> bool
```

### supports_tools

*Rust Method* — `src/llm/provider/gemini.rs#L647-L649`

_private_

```
fn supports_tools(&self) -> bool
```

### supports_vision

*Rust Method* — `src/llm/provider/gemini.rs#L651-L654`

_private_

```
fn supports_vision(&self) -> bool
```

### stream_url

*Rust Method* — `src/llm/provider/gemini.rs#L105-L110`

_private_

```
fn stream_url(&self, model: &str) -> String
```

**Called by:** stream

### to_gemini_request

*Rust Method* — `src/llm/provider/gemini.rs#L113-L261`

_private_

```
fn to_gemini_request(&self, request: &LLMRequest) -> GeminiRequest
```

**Calls:** gemini_role, is_empty, text

**Called by:** complete, stream, test_to_gemini_request_maps_system_and_tools, test_to_gemini_request_tool_result_uses_function_name, test_thinking_config_forwarded, test_json_mode_sets_response_mime_type, test_full_json_schema_sets_response_schema, test_inline_image_becomes_inline_data_part, test_image_url_source_is_skipped_without_panicking

### with_base_url

*Rust Method* — `src/llm/provider/gemini.rs#L66-L70`

```
pub fn with_base_url(api_key: String, base_url: String) -> Self
```

### with_default_model

*Rust Method* — `src/llm/provider/gemini.rs#L73-L76`

```
pub fn with_default_model(mut self, model: String) -> Self
```

### build_gemini_error

*Rust Function* — `src/llm/provider/gemini.rs#L355-L397`

_private_

```
fn build_gemini_error( status: u16, retry_after: Option<u64>, error_body: Option<GeminiErrorResponse>, ) -> ProviderError
```

**Called by:** handle_error, test_build_gemini_error_rate_limit_with_retry_after, test_build_gemini_error_rate_limit_without_retry_after, test_build_gemini_error_rate_limit_no_body, test_build_gemini_error_api_error_with_body, test_build_gemini_error_no_body_falls_back_to_unknown

### gemini_role

*Rust Function* — `src/llm/provider/gemini.rs#L399-L404`

_private_

```
fn gemini_role(role: &Role) -> &'static str
```

**Called by:** to_gemini_request

### parse_gemini_sse

*Rust Function* — `src/llm/provider/gemini.rs#L412-L538`

_private_

```
fn parse_gemini_sse(text: &str, model: &str) -> Vec<StreamEvent>
```

**Calls:** is_empty, next

**Called by:** stream, test_parse_gemini_sse_text_response, test_parse_gemini_sse_thinking_part, test_parse_gemini_sse_function_call, test_parse_gemini_sse_max_tokens, test_parse_gemini_sse_skips_malformed_lines, test_parse_gemini_sse_ignores_non_data_lines

### test_build_gemini_error_api_error_with_body

*Rust Function* — `src/llm/provider/gemini.rs#L1227-L1247`

_private_

```
fn test_build_gemini_error_api_error_with_body()
```

**Calls:** build_gemini_error

### test_build_gemini_error_no_body_falls_back_to_unknown

*Rust Function* — `src/llm/provider/gemini.rs#L1250-L1261`

_private_

```
fn test_build_gemini_error_no_body_falls_back_to_unknown()
```

**Calls:** build_gemini_error

### test_build_gemini_error_rate_limit_no_body

*Rust Function* — `src/llm/provider/gemini.rs#L1221-L1224`

_private_

```
fn test_build_gemini_error_rate_limit_no_body()
```

**Calls:** build_gemini_error

### test_build_gemini_error_rate_limit_with_retry_after

*Rust Function* — `src/llm/provider/gemini.rs#L1188-L1203`

_private_

```
fn test_build_gemini_error_rate_limit_with_retry_after()
```

**Calls:** build_gemini_error

### test_build_gemini_error_rate_limit_without_retry_after

*Rust Function* — `src/llm/provider/gemini.rs#L1206-L1218`

_private_

```
fn test_build_gemini_error_rate_limit_without_retry_after()
```

**Calls:** build_gemini_error

### test_calculate_cost_all_known_models

*Rust Function* — `src/llm/provider/gemini.rs#L1169-L1183`

_private_

```
fn test_calculate_cost_all_known_models()
```

### test_calculate_cost_gemini_flash

*Rust Function* — `src/llm/provider/gemini.rs#L936-L941`

_private_

```
fn test_calculate_cost_gemini_flash()
```

### test_context_window

*Rust Function* — `src/llm/provider/gemini.rs#L919-L924`

_private_

```
fn test_context_window()
```

### test_context_window_all_known_models

*Rust Function* — `src/llm/provider/gemini.rs#L1148-L1166`

_private_

```
fn test_context_window_all_known_models()
```

### test_custom_default_model

*Rust Function* — `src/llm/provider/gemini.rs#L903-L907`

_private_

```
fn test_custom_default_model()
```

### test_from_gemini_response_maps_thinking

*Rust Function* — `src/llm/provider/gemini.rs#L1037-L1066`

_private_

```
fn test_from_gemini_response_maps_thinking()
```

**Calls:** from_gemini_response

### test_from_gemini_response_maps_tool_use

*Rust Function* — `src/llm/provider/gemini.rs#L1003-L1034`

_private_

```
fn test_from_gemini_response_maps_tool_use()
```

**Calls:** from_gemini_response

### test_full_json_schema_sets_response_schema

*Rust Function* — `src/llm/provider/gemini.rs#L1098-L1110`

_private_

```
fn test_full_json_schema_sets_response_schema()
```

**Calls:** with_response_format, to_gemini_request

### test_gemini_provider_creation

*Rust Function* — `src/llm/provider/gemini.rs#L895-L900`

_private_

```
fn test_gemini_provider_creation()
```

### test_gemma_cost_is_free

*Rust Function* — `src/llm/provider/gemini.rs#L927-L933`

_private_

```
fn test_gemma_cost_is_free()
```

### test_image_url_source_is_skipped_without_panicking

*Rust Function* — `src/llm/provider/gemini.rs#L1132-L1145`

_private_

```
fn test_image_url_source_is_skipped_without_panicking()
```

**Calls:** to_gemini_request

### test_inline_image_becomes_inline_data_part

*Rust Function* — `src/llm/provider/gemini.rs#L1113-L1129`

_private_

```
fn test_inline_image_becomes_inline_data_part()
```

**Calls:** to_gemini_request

### test_json_mode_sets_response_mime_type

*Rust Function* — `src/llm/provider/gemini.rs#L1084-L1095`

_private_

```
fn test_json_mode_sets_response_mime_type()
```

**Calls:** with_response_format, to_gemini_request

### test_parse_gemini_sse_function_call

*Rust Function* — `src/llm/provider/gemini.rs#L1306-L1325`

_private_

```
fn test_parse_gemini_sse_function_call()
```

**Calls:** parse_gemini_sse

### test_parse_gemini_sse_ignores_non_data_lines

*Rust Function* — `src/llm/provider/gemini.rs#L1352-L1362`

_private_

```
fn test_parse_gemini_sse_ignores_non_data_lines()
```

**Calls:** parse_gemini_sse

### test_parse_gemini_sse_max_tokens

*Rust Function* — `src/llm/provider/gemini.rs#L1328-L1336`

_private_

```
fn test_parse_gemini_sse_max_tokens()
```

**Calls:** parse_gemini_sse

### test_parse_gemini_sse_skips_malformed_lines

*Rust Function* — `src/llm/provider/gemini.rs#L1339-L1349`

_private_

```
fn test_parse_gemini_sse_skips_malformed_lines()
```

**Calls:** parse_gemini_sse

### test_parse_gemini_sse_text_response

*Rust Function* — `src/llm/provider/gemini.rs#L1266-L1290`

_private_

```
fn test_parse_gemini_sse_text_response()
```

**Calls:** parse_gemini_sse

### test_parse_gemini_sse_thinking_part

*Rust Function* — `src/llm/provider/gemini.rs#L1293-L1303`

_private_

```
fn test_parse_gemini_sse_thinking_part()
```

**Calls:** parse_gemini_sse

### test_role_mapping

*Rust Function* — `src/llm/provider/gemini.rs#L944-L948`

_private_

```
fn test_role_mapping()
```

### test_supported_models_include_gemma

*Rust Function* — `src/llm/provider/gemini.rs#L910-L916`

_private_

```
fn test_supported_models_include_gemma()
```

### test_thinking_config_forwarded

*Rust Function* — `src/llm/provider/gemini.rs#L1069-L1081`

_private_

```
fn test_thinking_config_forwarded()
```

**Calls:** to_gemini_request

### test_to_gemini_request_maps_system_and_tools

*Rust Function* — `src/llm/provider/gemini.rs#L951-L968`

_private_

```
fn test_to_gemini_request_maps_system_and_tools()
```

**Calls:** with_system, with_tools, to_gemini_request

### test_to_gemini_request_tool_result_uses_function_name

*Rust Function* — `src/llm/provider/gemini.rs#L971-L1000`

_private_

```
fn test_to_gemini_request_tool_result_uses_function_name()
```

**Calls:** to_gemini_request

### detects_known_vision_models

*Rust Function* — `src/llm/provider/model_hints.rs#L35-L40`

_private_

```
fn detects_known_vision_models()
```

### is_vision_model

*Rust Function* — `src/llm/provider/model_hints.rs#L10-L28`

```
pub fn is_vision_model(model_name: &str) -> bool
```

**Called by:** supports_vision, supports_vision

### rejects_non_vision_models

*Rust Function* — `src/llm/provider/model_hints.rs#L43-L46`

_private_

```
fn rejects_non_vision_models()
```

### from_config

*Rust Method* — `src/llm/provider/ollama.rs#L84-L107`

```
pub fn from_config( temperature: Option<f32>, top_p: Option<f32>, top_k: Option<u32>, num_ctx: Option<u32>, keep_alive: Option<&str>, think: Option<&str>, ) -> Self
```

**Calls:** parse_keep_alive

**Called by:** ollama_provider_from_config, per_model_override_wins_over_provider_default_for_that_model, per_model_override_falls_back_field_by_field, context_window_reflects_the_per_model_num_ctx_that_is_actually_requested, per_model_think_false_is_sent_when_request_has_no_thinking

### default_local

*Rust Method* — `src/llm/provider/ollama.rs#L157-L159`

```
pub fn default_local() -> Self
```

**Called by:** streamed_ollama_tool_call_survives_drain, test_ollama_provider_creation, test_with_default_model, per_model_override_wins_over_provider_default_for_that_model, per_model_override_falls_back_field_by_field, overrides_for_returns_provider_defaults_when_no_per_model_map, context_window_reflects_the_per_model_num_ctx_that_is_actually_requested, test_validate_model_always_true, test_supported_models_includes_gemma4, test_supported_models_includes_ornith, test_context_window_default_and_custom, test_calculate_cost_is_always_zero, test_supports_vision_detection, test_to_ollama_request_maps_common_fields, recovered_tool_call_becomes_a_tool_use_block, fenced_call_in_prose_becomes_a_tool_use_block, from_ollama_response_plain_text_with_final_data, from_ollama_response_without_final_data_has_zero_usage_and_no_perf, from_ollama_response_extracts_tool_calls, streamed_tool_call_reaches_caller, from_ollama_response_uses_explicit_thinking_field, from_ollama_response_falls_back_to_think_tags, to_ollama_request_maps_tool_messages, to_ollama_request_maps_thinking_and_response_format, per_model_think_false_is_sent_when_request_has_no_thinking, request_thinking_wins_over_configured_think, invalid_think_value_is_ignored, to_ollama_request_embeds_base64_image

### from_ollama_response

*Rust Method* — `src/llm/provider/ollama.rs#L420-L508`

_private_

```
fn from_ollama_response( &self, response: ChatMessageResponse, offered_tools: &[Tool], ) -> LLMResponse
```

**Calls:** is_empty, extract_think_tags, tool_call_from_content, perf_metrics_from_final_data

**Called by:** complete, recovered_tool_call_becomes_a_tool_use_block, fenced_call_in_prose_becomes_a_tool_use_block, from_ollama_response_plain_text_with_final_data, from_ollama_response_without_final_data_has_zero_usage_and_no_perf, from_ollama_response_extracts_tool_calls, from_ollama_response_uses_explicit_thinking_field, from_ollama_response_falls_back_to_think_tags

### new

*Rust Method* — `src/llm/provider/ollama.rs#L166-L188`

```
pub fn new(host: impl Into<String>) -> Self
```

### overrides_for

*Rust Method* — `src/llm/provider/ollama.rs#L212-L241`

_private_

```
fn overrides_for(&self, model: &str) -> ModelOverrides
```

**Calls:** is_empty

**Called by:** to_ollama_request, context_window, per_model_override_wins_over_provider_default_for_that_model, per_model_override_falls_back_field_by_field, overrides_for_returns_provider_defaults_when_no_per_model_map

### calculate_cost

*Rust Method* — `src/llm/provider/ollama.rs#L774-L777`

_private_

```
fn calculate_cost(&self, _model: &str, _input_tokens: u32, _output_tokens: u32) -> f64
```

### complete

*Rust Method* — `src/llm/provider/ollama.rs#L513-L541`

_private_

```
async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>
```

**Calls:** to_ollama_request, from_ollama_response

### context_window

*Rust Method* — `src/llm/provider/ollama.rs#L756-L772`

_private_

```
fn context_window(&self, model: &str) -> Option<u32>
```

**Calls:** overrides_for

### default_model

*Rust Method* — `src/llm/provider/ollama.rs#L731-L733`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/llm/provider/ollama.rs#L727-L729`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/llm/provider/ollama.rs#L543-L713`

_private_

```
async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>
```

**Calls:** to_ollama_request, next, is_empty, maybe_tool_call_json, collect_tool_calls, perf_metrics_from_final_data, tool_call_from_content, stop_reason_for

### supported_models

*Rust Method* — `src/llm/provider/ollama.rs#L735-L749`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### supports_streaming

*Rust Method* — `src/llm/provider/ollama.rs#L715-L717`

_private_

```
fn supports_streaming(&self) -> bool
```

### supports_tools

*Rust Method* — `src/llm/provider/ollama.rs#L719-L721`

_private_

```
fn supports_tools(&self) -> bool
```

### supports_vision

*Rust Method* — `src/llm/provider/ollama.rs#L723-L725`

_private_

```
fn supports_vision(&self) -> bool
```

**Calls:** is_vision_model

### validate_model

*Rust Method* — `src/llm/provider/ollama.rs#L751-L754`

_private_

```
fn validate_model(&self, _model: &str) -> bool
```

### to_ollama_request

*Rust Method* — `src/llm/provider/ollama.rs#L281-L415`

_private_

```
fn to_ollama_request(&self, request: LLMRequest) -> ChatMessageRequest
```

**Calls:** system, is_empty, overrides_for

**Called by:** complete, stream, test_to_ollama_request_maps_common_fields, to_ollama_request_maps_tool_messages, to_ollama_request_maps_thinking_and_response_format, per_model_think_false_is_sent_when_request_has_no_thinking, request_thinking_wins_over_configured_think, to_ollama_request_embeds_base64_image

### with_default_model

*Rust Method* — `src/llm/provider/ollama.rs#L244-L247`

```
pub fn with_default_model(mut self, model: String) -> Self
```

### with_keep_alive

*Rust Method* — `src/llm/provider/ollama.rs#L253-L259`

```
pub fn with_keep_alive(mut self, keep_alive: &str) -> Self
```

**Calls:** parse_keep_alive

**Called by:** ollama_provider_from_config

### with_num_ctx

*Rust Method* — `src/llm/provider/ollama.rs#L262-L265`

```
pub fn with_num_ctx(mut self, num_ctx: u32) -> Self
```

**Called by:** ollama_provider_from_config, overrides_for_returns_provider_defaults_when_no_per_model_map, context_window_reflects_the_per_model_num_ctx_that_is_actually_requested, test_context_window_default_and_custom

### with_per_model

*Rust Method* — `src/llm/provider/ollama.rs#L192-L198`

```
pub fn with_per_model( mut self, per_model: std::collections::HashMap<String, ModelOverrides>, ) -> Self
```

**Called by:** ollama_provider_from_config, per_model_override_wins_over_provider_default_for_that_model, per_model_override_falls_back_field_by_field, context_window_reflects_the_per_model_num_ctx_that_is_actually_requested, per_model_think_false_is_sent_when_request_has_no_thinking

### with_sampling

*Rust Method* — `src/llm/provider/ollama.rs#L268-L278`

```
pub fn with_sampling( mut self, temperature: Option<f32>, top_p: Option<f32>, top_k: Option<u32>, ) -> Self
```

### with_think

*Rust Method* — `src/llm/provider/ollama.rs#L203-L206`

```
pub fn with_think(mut self, think: &str) -> Self
```

**Calls:** parse_think

**Called by:** ollama_provider_from_config, request_thinking_wins_over_configured_think, invalid_think_value_is_ignored

### bash_tool

*Rust Function* — `src/llm/provider/ollama.rs#L1286-L1296`

_private_

```
fn bash_tool() -> Tool
```

**Called by:** tool_call_printed_as_content_is_recovered, tool_call_in_a_json_fence_is_recovered, tool_call_in_a_fence_embedded_in_prose_is_recovered, first_of_several_fenced_calls_is_recovered, fenced_non_tool_json_is_not_recovered, prose_is_never_mistaken_for_a_tool_call, recovered_tool_call_becomes_a_tool_use_block, fenced_call_in_prose_becomes_a_tool_use_block

### collect_tool_calls

*Rust Function* — `src/llm/provider/ollama.rs#L785-L790`

_private_

```
fn collect_tool_calls(tool_calls: &[ToolCall]) -> Vec<(String, serde_json::Value)>
```

**Called by:** stream, streamed_tool_calls_arrive_before_the_done_chunk

### context_window_reflects_the_per_model_num_ctx_that_is_actually_requested

*Rust Function* — `src/llm/provider/ollama.rs#L1114-L1138`

_private_

```
fn context_window_reflects_the_per_model_num_ctx_that_is_actually_requested()
```

**Calls:** from_config, default_local, with_num_ctx, with_per_model

### fenced_call_in_prose_becomes_a_tool_use_block

*Rust Function* — `src/llm/provider/ollama.rs#L1430-L1452`

_private_

```
fn fenced_call_in_prose_becomes_a_tool_use_block()
```

**Calls:** default_local, mock_response, assistant, from_ollama_response, bash_tool

### fenced_json_blocks

*Rust Function* — `src/llm/provider/ollama.rs#L862-L880`

_private_

```
fn fenced_json_blocks(content: &str) -> Vec<&str>
```

**Called by:** tool_call_from_content

### fenced_non_tool_json_is_not_recovered

*Rust Function* — `src/llm/provider/ollama.rs#L1355-L1360`

_private_

```
fn fenced_non_tool_json_is_not_recovered()
```

**Calls:** bash_tool

### first_of_several_fenced_calls_is_recovered

*Rust Function* — `src/llm/provider/ollama.rs#L1340-L1350`

_private_

```
fn first_of_several_fenced_calls_is_recovered()
```

**Calls:** bash_tool, tool_call_from_content

### from_ollama_response_extracts_tool_calls

*Rust Function* — `src/llm/provider/ollama.rs#L1494-L1515`

_private_

```
fn from_ollama_response_extracts_tool_calls()
```

**Calls:** default_local, assistant, mock_response, from_ollama_response

### from_ollama_response_falls_back_to_think_tags

*Rust Function* — `src/llm/provider/ollama.rs#L1615-L1632`

_private_

```
fn from_ollama_response_falls_back_to_think_tags()
```

**Calls:** default_local, assistant, mock_response, from_ollama_response

### from_ollama_response_plain_text_with_final_data

*Rust Function* — `src/llm/provider/ollama.rs#L1455-L1478`

_private_

```
fn from_ollama_response_plain_text_with_final_data()
```

**Calls:** default_local, mock_response, assistant, from_ollama_response

### from_ollama_response_uses_explicit_thinking_field

*Rust Function* — `src/llm/provider/ollama.rs#L1596-L1612`

_private_

```
fn from_ollama_response_uses_explicit_thinking_field()
```

**Calls:** default_local, assistant, mock_response, from_ollama_response

### from_ollama_response_without_final_data_has_zero_usage_and_no_perf

*Rust Function* — `src/llm/provider/ollama.rs#L1481-L1491`

_private_

```
fn from_ollama_response_without_final_data_has_zero_usage_and_no_perf()
```

**Calls:** default_local, mock_response, assistant, from_ollama_response

### invalid_think_value_is_ignored

*Rust Function* — `src/llm/provider/ollama.rs#L1769-L1773`

_private_

```
fn invalid_think_value_is_ignored()
```

**Calls:** default_local, with_think

### map_ollama_error

*Rust Function* — `src/llm/provider/ollama.rs#L995-L1036`

_private_

```
fn map_ollama_error(err: OllamaError) -> ProviderError
```

**Called by:** test_map_ollama_error_not_found, model_not_found_error_is_unwrapped_and_actionable

### maybe_tool_call_json

*Rust Function* — `src/llm/provider/ollama.rs#L807-L810`

_private_

```
fn maybe_tool_call_json(text: &str) -> bool
```

**Calls:** is_empty

**Called by:** stream

### mock_response

*Rust Function* — `src/llm/provider/ollama.rs#L1275-L1284`

_private_

```
fn mock_response(message: ChatMessage, done: bool) -> ChatMessageResponse
```

**Called by:** recovered_tool_call_becomes_a_tool_use_block, fenced_call_in_prose_becomes_a_tool_use_block, from_ollama_response_plain_text_with_final_data, from_ollama_response_without_final_data_has_zero_usage_and_no_perf, from_ollama_response_extracts_tool_calls, from_ollama_response_uses_explicit_thinking_field, from_ollama_response_falls_back_to_think_tags

### model_not_found_error_is_unwrapped_and_actionable

*Rust Function* — `src/llm/provider/ollama.rs#L1239-L1254`

_private_

```
fn model_not_found_error_is_unwrapped_and_actionable()
```

**Calls:** map_ollama_error

### only_json_like_content_is_withheld_from_streaming

*Rust Function* — `src/llm/provider/ollama.rs#L1387-L1394`

_private_

```
fn only_json_like_content_is_withheld_from_streaming()
```

### overrides_for_returns_provider_defaults_when_no_per_model_map

*Rust Function* — `src/llm/provider/ollama.rs#L1100-L1111`

_private_

```
fn overrides_for_returns_provider_defaults_when_no_per_model_map()
```

**Calls:** default_local, with_num_ctx, overrides_for

### parse_keep_alive

*Rust Function* — `src/llm/provider/ollama.rs#L958-L976`

_private_

```
fn parse_keep_alive(s: &str) -> Option<KeepAlive>
```

**Calls:** len

**Called by:** from_config, with_keep_alive

### parse_think

*Rust Function* — `src/llm/provider/ollama.rs#L112-L127`

_private_

```
fn parse_think(s: &str) -> Option<ThinkType>
```

**Called by:** with_think

### parse_tool_call_object

*Rust Function* — `src/llm/provider/ollama.rs#L885-L921`

_private_

```
fn parse_tool_call_object(text: &str, offered: &[Tool]) -> Option<(String, serde_json::Value)>
```

**Calls:** from_str

**Called by:** tool_call_from_content

### per_model_override_falls_back_field_by_field

*Rust Function* — `src/llm/provider/ollama.rs#L1082-L1097`

_private_

```
fn per_model_override_falls_back_field_by_field()
```

**Calls:** from_config, default_local, with_per_model, overrides_for

### per_model_override_wins_over_provider_default_for_that_model

*Rust Function* — `src/llm/provider/ollama.rs#L1057-L1079`

_private_

```
fn per_model_override_wins_over_provider_default_for_that_model()
```

**Calls:** from_config, default_local, with_per_model, overrides_for

### per_model_think_false_is_sent_when_request_has_no_thinking

*Rust Function* — `src/llm/provider/ollama.rs#L1736-L1756`

_private_

```
fn per_model_think_false_is_sent_when_request_has_no_thinking()
```

**Calls:** from_config, default_local, with_per_model, to_ollama_request

### perf_metrics_from_final_data

*Rust Function* — `src/llm/provider/ollama.rs#L980-L989`

_private_

```
fn perf_metrics_from_final_data(final_data: &ChatMessageFinalResponseData) -> PerfMetrics
```

**Called by:** from_ollama_response, stream, test_perf_metrics_from_final_data

### prose_is_never_mistaken_for_a_tool_call

*Rust Function* — `src/llm/provider/ollama.rs#L1365-L1382`

_private_

```
fn prose_is_never_mistaken_for_a_tool_call()
```

**Calls:** bash_tool

### recovered_tool_call_becomes_a_tool_use_block

*Rust Function* — `src/llm/provider/ollama.rs#L1400-L1424`

_private_

```
fn recovered_tool_call_becomes_a_tool_use_block()
```

**Calls:** default_local, mock_response, assistant, from_ollama_response, bash_tool

### request_thinking_wins_over_configured_think

*Rust Function* — `src/llm/provider/ollama.rs#L1760-L1765`

_private_

```
fn request_thinking_wins_over_configured_think()
```

**Calls:** default_local, with_think, to_ollama_request

### stop_reason_for

*Rust Function* — `src/llm/provider/ollama.rs#L793-L799`

_private_

```
fn stop_reason_for(tool_calls: &[(String, serde_json::Value)]) -> StopReason
```

**Calls:** is_empty

**Called by:** stream

### stream_without_tool_calls_ends_the_turn

*Rust Function* — `src/llm/provider/ollama.rs#L1541-L1543`

_private_

```
fn stream_without_tool_calls_ends_the_turn()
```

### streamed_tool_call_reaches_caller

*Rust Function* — `src/llm/provider/ollama.rs#L1551-L1593`

_private_

```
async fn streamed_tool_call_reaches_caller()
```

**Calls:** default_local, with_tools, next

### streamed_tool_calls_arrive_before_the_done_chunk

*Rust Function* — `src/llm/provider/ollama.rs#L1521-L1538`

_private_

```
fn streamed_tool_calls_arrive_before_the_done_chunk()
```

**Calls:** collect_tool_calls

### test_calculate_cost_is_always_zero

*Rust Function* — `src/llm/provider/ollama.rs#L1178-L1181`

_private_

```
fn test_calculate_cost_is_always_zero()
```

**Calls:** default_local

### test_context_window_default_and_custom

*Rust Function* — `src/llm/provider/ollama.rs#L1169-L1175`

_private_

```
fn test_context_window_default_and_custom()
```

**Calls:** default_local, with_num_ctx

### test_invalid_host_falls_back_to_default

*Rust Function* — `src/llm/provider/ollama.rs#L1141-L1145`

_private_

```
fn test_invalid_host_falls_back_to_default()
```

### test_map_ollama_error_not_found

*Rust Function* — `src/llm/provider/ollama.rs#L1226-L1231`

_private_

```
fn test_map_ollama_error_not_found()
```

**Calls:** map_ollama_error

### test_ollama_provider_creation

*Rust Function* — `src/llm/provider/ollama.rs#L1043-L1047`

_private_

```
fn test_ollama_provider_creation()
```

**Calls:** default_local

### test_parse_keep_alive

*Rust Function* — `src/llm/provider/ollama.rs#L1193-L1204`

_private_

```
fn test_parse_keep_alive()
```

### test_perf_metrics_from_final_data

*Rust Function* — `src/llm/provider/ollama.rs#L1207-L1223`

_private_

```
fn test_perf_metrics_from_final_data()
```

**Calls:** perf_metrics_from_final_data

### test_supported_models_includes_gemma4

*Rust Function* — `src/llm/provider/ollama.rs#L1154-L1159`

_private_

```
fn test_supported_models_includes_gemma4()
```

**Calls:** default_local

### test_supported_models_includes_ornith

*Rust Function* — `src/llm/provider/ollama.rs#L1162-L1166`

_private_

```
fn test_supported_models_includes_ornith()
```

**Calls:** default_local

### test_supports_vision_detection

*Rust Function* — `src/llm/provider/ollama.rs#L1184-L1190`

_private_

```
fn test_supports_vision_detection()
```

**Calls:** default_local

### test_to_ollama_request_maps_common_fields

*Rust Function* — `src/llm/provider/ollama.rs#L1257-L1273`

_private_

```
fn test_to_ollama_request_maps_common_fields()
```

**Calls:** default_local, with_system, with_temperature, with_top_p, with_seed, with_stop, with_max_tokens, to_ollama_request

### test_validate_model_always_true

*Rust Function* — `src/llm/provider/ollama.rs#L1148-L1151`

_private_

```
fn test_validate_model_always_true()
```

**Calls:** default_local

### test_with_default_model

*Rust Function* — `src/llm/provider/ollama.rs#L1050-L1054`

_private_

```
fn test_with_default_model()
```

**Calls:** default_local

### to_ollama_format

*Rust Function* — `src/llm/provider/ollama.rs#L946-L953`

_private_

```
fn to_ollama_format(value: &serde_json::Value) -> Option<FormatType>
```

### to_ollama_format_json_object_marker

*Rust Function* — `src/llm/provider/ollama.rs#L1666-L1669`

_private_

```
fn to_ollama_format_json_object_marker()
```

### to_ollama_format_structured_schema

*Rust Function* — `src/llm/provider/ollama.rs#L1672-L1681`

_private_

```
fn to_ollama_format_structured_schema()
```

### to_ollama_request_embeds_base64_image

*Rust Function* — `src/llm/provider/ollama.rs#L1776-L1803`

_private_

```
fn to_ollama_request_embeds_base64_image()
```

**Calls:** default_local, to_ollama_request

### to_ollama_request_maps_thinking_and_response_format

*Rust Function* — `src/llm/provider/ollama.rs#L1720-L1729`

_private_

```
fn to_ollama_request_maps_thinking_and_response_format()
```

**Calls:** default_local, with_response_format, to_ollama_request

### to_ollama_request_maps_tool_messages

*Rust Function* — `src/llm/provider/ollama.rs#L1684-L1717`

_private_

```
fn to_ollama_request_maps_tool_messages()
```

**Calls:** default_local, to_ollama_request

### to_ollama_tool

*Rust Function* — `src/llm/provider/ollama.rs#L925-L942`

_private_

```
fn to_ollama_tool(tool: &Tool) -> ToolInfo
```

**Called by:** to_ollama_tool_converts_valid_schema, to_ollama_tool_falls_back_on_invalid_schema

### to_ollama_tool_converts_valid_schema

*Rust Function* — `src/llm/provider/ollama.rs#L1635-L1648`

_private_

```
fn to_ollama_tool_converts_valid_schema()
```

**Calls:** to_ollama_tool

### to_ollama_tool_falls_back_on_invalid_schema

*Rust Function* — `src/llm/provider/ollama.rs#L1651-L1663`

_private_

```
fn to_ollama_tool_falls_back_on_invalid_schema()
```

**Calls:** to_ollama_tool

### tool_call_from_content

*Rust Function* — `src/llm/provider/ollama.rs#L825-L857`

_private_

```
fn tool_call_from_content(content: &str, offered: &[Tool]) -> Option<(String, serde_json::Value)>
```

**Calls:** parse_tool_call_object, fenced_json_blocks

**Called by:** from_ollama_response, stream, tool_call_printed_as_content_is_recovered, tool_call_in_a_json_fence_is_recovered, tool_call_in_a_fence_embedded_in_prose_is_recovered, first_of_several_fenced_calls_is_recovered

### tool_call_in_a_fence_embedded_in_prose_is_recovered

*Rust Function* — `src/llm/provider/ollama.rs#L1325-L1335`

_private_

```
fn tool_call_in_a_fence_embedded_in_prose_is_recovered()
```

**Calls:** bash_tool, tool_call_from_content

### tool_call_in_a_json_fence_is_recovered

*Rust Function* — `src/llm/provider/ollama.rs#L1314-L1319`

_private_

```
fn tool_call_in_a_json_fence_is_recovered()
```

**Calls:** bash_tool, tool_call_from_content

### tool_call_printed_as_content_is_recovered

*Rust Function* — `src/llm/provider/ollama.rs#L1302-L1311`

_private_

```
fn tool_call_printed_as_content_is_recovered()
```

**Calls:** bash_tool, tool_call_from_content

### fraction

*Rust Method* — `src/llm/provider/ollama_models.rs#L47-L51`

```
pub fn fraction(&self) -> Option<f64>
```

### is_success

*Rust Method* — `src/llm/provider/ollama_models.rs#L42-L44`

```
pub fn is_success(&self) -> bool
```

**Called by:** complete, stream, complete, stream, complete, stream, complete, stream, execute, execute, execute

### client_for

*Rust Function* — `src/llm/provider/ollama_models.rs#L63-L65`

_private_

```
fn client_for(host: &str) -> Result<Ollama>
```

**Calls:** with_context

**Called by:** list_models, show_model, delete_model, pull_model, generate_embeddings, invalid_host_returns_error

### delete_model

*Rust Function* — `src/llm/provider/ollama_models.rs#L102-L109`

```
pub async fn delete_model(host: &str, model_name: &str) -> Result<()>
```

**Calls:** client_for, with_context

**Called by:** cmd_ollama, delete_model_succeeds_on_2xx, spawn_delete

### delete_model_succeeds_on_2xx

*Rust Function* — `src/llm/provider/ollama_models.rs#L305-L310`

_private_

```
async fn delete_model_succeeds_on_2xx()
```

**Calls:** mock_server, delete_model

### embeddings_request_serializes_model_and_input

*Rust Function* — `src/llm/provider/ollama_models.rs#L209-L219`

_private_

```
fn embeddings_request_serializes_model_and_input()
```

### embeddings_request_single_input_is_not_wrapped_in_array

*Rust Function* — `src/llm/provider/ollama_models.rs#L222-L231`

_private_

```
fn embeddings_request_single_input_is_not_wrapped_in_array()
```

### generate_embeddings

*Rust Function* — `src/llm/provider/ollama_models.rs#L148-L163`

```
pub async fn generate_embeddings( host: &str, model_name: &str, input: Vec<String>, ) -> Result<Vec<Vec<f32>>>
```

**Calls:** client_for, with_context

**Called by:** cmd_ollama, generate_embeddings_parses_response

### generate_embeddings_parses_response

*Rust Function* — `src/llm/provider/ollama_models.rs#L341-L348`

_private_

```
async fn generate_embeddings_parses_response()
```

**Calls:** mock_server, generate_embeddings

### invalid_host_returns_error

*Rust Function* — `src/llm/provider/ollama_models.rs#L203-L206`

_private_

```
fn invalid_host_returns_error()
```

**Calls:** client_for

### list_models

*Rust Function* — `src/llm/provider/ollama_models.rs#L68-L83`

```
pub async fn list_models(host: &str) -> Result<Vec<LocalModelInfo>>
```

**Calls:** client_for

**Called by:** cmd_ollama, list_models_parses_tags_response, fetch_installed_models

### list_models_parses_tags_response

*Rust Function* — `src/llm/provider/ollama_models.rs#L276-L287`

_private_

```
async fn list_models_parses_tags_response()
```

**Calls:** mock_server, list_models

### mock_server

*Rust Function* — `src/llm/provider/ollama_models.rs#L237-L273`

_private_

```
async fn mock_server(body: String) -> String
```

**Called by:** list_models_parses_tags_response, show_model_parses_minimal_response, delete_model_succeeds_on_2xx, pull_model_forwards_progress_and_completes, generate_embeddings_parses_response

### pull_model

*Rust Function* — `src/llm/provider/ollama_models.rs#L116-L140`

```
pub async fn pull_model( host: &str, model_name: &str, progress_tx: UnboundedSender<PullProgress>, ) -> Result<()>
```

**Calls:** client_for, with_context, next

**Called by:** cmd_ollama, pull_model_forwards_progress_and_completes, spawn_pull

### pull_model_forwards_progress_and_completes

*Rust Function* — `src/llm/provider/ollama_models.rs#L313-L338`

_private_

```
async fn pull_model_forwards_progress_and_completes()
```

**Calls:** mock_server, pull_model

### pull_progress_fraction

*Rust Function* — `src/llm/provider/ollama_models.rs#L170-L178`

_private_

```
fn pull_progress_fraction()
```

### pull_progress_fraction_missing_data

*Rust Function* — `src/llm/provider/ollama_models.rs#L181-L189`

_private_

```
fn pull_progress_fraction_missing_data()
```

### pull_progress_is_success

*Rust Function* — `src/llm/provider/ollama_models.rs#L192-L200`

_private_

```
fn pull_progress_is_success()
```

### show_model

*Rust Function* — `src/llm/provider/ollama_models.rs#L86-L99`

```
pub async fn show_model(host: &str, model_name: &str) -> Result<ModelDetails>
```

**Calls:** client_for, with_context

**Called by:** cmd_ollama, show_model_parses_minimal_response

### show_model_parses_minimal_response

*Rust Function* — `src/llm/provider/ollama_models.rs#L290-L302`

_private_

```
async fn show_model_parses_minimal_response()
```

**Calls:** mock_server, show_model

### from_openai_response

*Rust Method* — `src/llm/provider/openai.rs#L348-L470`

_private_

```
fn from_openai_response(&self, response: OpenAIResponse) -> LLMResponse
```

**Calls:** next, is_empty, extract_think_tags, from_str

**Called by:** complete

### handle_error

*Rust Method* — `src/llm/provider/openai.rs#L473-L530`

_private_

```
async fn handle_error(&self, response: reqwest::Response) -> ProviderError
```

### headers

*Rust Method* — `src/llm/provider/openai.rs#L151-L185`

_private_

```
fn headers(&self) -> Result<reqwest::header::HeaderMap>
```

**Calls:** parse

### local

*Rust Method* — `src/llm/provider/openai.rs#L89-L105`

```
pub fn local(base_url: String) -> Self
```

### new

*Rust Method* — `src/llm/provider/openai.rs#L70-L86`

```
pub fn new(api_key: String) -> Self
```

### calculate_cost

*Rust Method* — `src/llm/provider/openai.rs#L935-L956`

_private_

```
fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64
```

### complete

*Rust Method* — `src/llm/provider/openai.rs#L535-L597`

_private_

```
async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>
```

**Calls:** len, to_openai_request, retry_with_backoff, is_success, from_openai_response

### context_window

*Rust Method* — `src/llm/provider/openai.rs#L920-L933`

_private_

```
fn context_window(&self, model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/llm/provider/openai.rs#L904-L908`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/llm/provider/openai.rs#L900-L902`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/llm/provider/openai.rs#L599-L883`

_private_

```
async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>
```

**Calls:** len, to_openai_request, retry_with_backoff, is_success, next, is_empty, from_str

### supported_models

*Rust Method* — `src/llm/provider/openai.rs#L910-L918`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### supports_streaming

*Rust Method* — `src/llm/provider/openai.rs#L885-L887`

_private_

```
fn supports_streaming(&self) -> bool
```

### supports_tools

*Rust Method* — `src/llm/provider/openai.rs#L889-L891`

_private_

```
fn supports_tools(&self) -> bool
```

### supports_vision

*Rust Method* — `src/llm/provider/openai.rs#L893-L898`

_private_

```
fn supports_vision(&self) -> bool
```

**Calls:** is_vision_model

### to_openai_request

*Rust Method* — `src/llm/provider/openai.rs#L188-L344`

_private_

```
fn to_openai_request(&self, request: LLMRequest) -> OpenAIRequest
```

**Calls:** is_empty

**Called by:** complete, stream, test_new_fields_forwarded_to_openai_request

### with_api_key_header

*Rust Method* — `src/llm/provider/openai.rs#L138-L141`

```
pub(crate) fn with_api_key_header(mut self) -> Self
```

**Called by:** new, with_endpoint, with_api_key_header_sends_api_key_not_bearer

### with_base_url

*Rust Method* — `src/llm/provider/openai.rs#L108-L124`

```
pub fn with_base_url(api_key: String, base_url: String) -> Self
```

### with_default_model

*Rust Method* — `src/llm/provider/openai.rs#L127-L130`

```
pub fn with_default_model(mut self, model: String) -> Self
```

### default_auth_style_still_sends_bearer

*Rust Function* — `src/llm/provider/openai.rs#L1192-L1202`

_private_

```
fn default_auth_style_still_sends_bearer()
```

### test_calculate_cost

*Rust Function* — `src/llm/provider/openai.rs#L1288-L1294`

_private_

```
fn test_calculate_cost()
```

### test_calculate_cost_unknown_model_returns_zero

*Rust Function* — `src/llm/provider/openai.rs#L1297-L1303`

_private_

```
fn test_calculate_cost_unknown_model_returns_zero()
```

### test_context_window

*Rust Function* — `src/llm/provider/openai.rs#L1213-L1224`

_private_

```
fn test_context_window()
```

### test_llm_request_new_fields

*Rust Function* — `src/llm/provider/openai.rs#L1248-L1264`

_private_

```
fn test_llm_request_new_fields()
```

**Calls:** with_top_p, with_seed, with_stop, with_frequency_penalty, with_presence_penalty, with_response_format

### test_local_provider_creation

*Rust Function* — `src/llm/provider/openai.rs#L1158-L1162`

_private_

```
fn test_local_provider_creation()
```

### test_new_fields_forwarded_to_openai_request

*Rust Function* — `src/llm/provider/openai.rs#L1267-L1285`

_private_

```
fn test_new_fields_forwarded_to_openai_request()
```

**Calls:** with_top_p, with_seed, with_stop, with_frequency_penalty, with_presence_penalty, with_response_format, to_openai_request

### test_openai_provider_creation

*Rust Function* — `src/llm/provider/openai.rs#L1151-L1155`

_private_

```
fn test_openai_provider_creation()
```

### test_supported_models

*Rust Function* — `src/llm/provider/openai.rs#L1205-L1210`

_private_

```
fn test_supported_models()
```

### test_supports_vision_detection

*Rust Function* — `src/llm/provider/openai.rs#L1227-L1245`

_private_

```
fn test_supports_vision_detection()
```

### test_tool_call_index_in_bounds

*Rust Function* — `src/llm/provider/openai.rs#L1306-L1311`

_private_

```
fn test_tool_call_index_in_bounds()
```

### tool_call_index_in_bounds

*Rust Function* — `src/llm/provider/openai.rs#L41-L43`

_private_

```
fn tool_call_index_in_bounds(idx: usize) -> bool
```

### with_api_key_header_sends_api_key_not_bearer

*Rust Function* — `src/llm/provider/openai.rs#L1171-L1188`

_private_

```
fn with_api_key_header_sends_api_key_not_bearer()
```

**Calls:** with_api_key_header

### build_client

*Rust Method* — `src/llm/provider/qwen.rs#L213-L221`

_private_

```
fn build_client() -> Client
```

**Called by:** local, with_base_url

### clean_incomplete_markers

*Rust Method* — `src/llm/provider/qwen.rs#L601-L620`

_private_

```
fn clean_incomplete_markers(&self, text: &str) -> String
```

**Calls:** len

**Called by:** from_qwen_response, test_clean_incomplete_markers

### dashscope_cn

*Rust Method* — `src/llm/provider/qwen.rs#L128-L130`

```
pub fn dashscope_cn(api_key: String) -> Self
```

**Called by:** try_create_qwen

### dashscope_intl

*Rust Method* — `src/llm/provider/qwen.rs#L123-L125`

```
pub fn dashscope_intl(api_key: String) -> Self
```

**Called by:** try_create_qwen, test_qwen_provider_creation, test_supported_models, test_context_window, test_calculate_cost_cloud, test_calculate_cost_unknown_cloud_model_returns_zero, test_sampling_defaults_dashscope_omits_vendor_extensions, test_sampling_config_override_wins_over_defaults

### default_sampling

*Rust Method* — `src/llm/provider/qwen.rs#L918-L936`

_private_

```
fn default_sampling( model: &str, thinking_enabled: bool, ) -> (Option<f32>, Option<u32>, Option<f32>)
```

**Called by:** to_qwen_request

### expand_span_over_adjacent_fences

*Rust Method* — `src/llm/provider/qwen.rs#L412-L425`

_private_

```
fn expand_span_over_adjacent_fences(text: &str, start: usize, end: usize) -> (usize, usize)
```

**Calls:** len

**Called by:** parse_fallback_tool_calls

### extract_thinking

*Rust Method* — `src/llm/provider/qwen.rs#L481-L500`

_private_

```
fn extract_thinking(&self, text: &str) -> (Option<String>, String)
```

**Calls:** find_after

**Called by:** from_qwen_response, test_thinking_extraction, test_thinking_extraction_out_of_order_tags_does_not_panic

### find_json_objects

*Rust Method* — `src/llm/provider/qwen.rs#L350-L405`

_private_

```
fn find_json_objects(text: &str) -> Vec<(usize, usize, serde_json::Value)>
```

**Calls:** len

**Called by:** parse_fallback_tool_calls, test_find_json_objects_recovers_nested_object_after_failed_outer_parse

### format_hermes_tools

*Rust Method* — `src/llm/provider/qwen.rs#L270-L291`

_private_

```
fn format_hermes_tools(&self, tools: &[Tool]) -> String
```

**Called by:** to_qwen_request, test_hermes_tools_format

### format_native_qwen_result

*Rust Method* — `src/llm/provider/qwen.rs#L596-L598`

_private_

```
fn format_native_qwen_result(&self, result: &str) -> String
```

**Called by:** to_qwen_request, test_native_qwen_result_format

### format_native_qwen_tools

*Rust Method* — `src/llm/provider/qwen.rs#L503-L534`

_private_

```
fn format_native_qwen_tools(&self, tools: &[Tool]) -> String
```

**Called by:** to_qwen_request, test_native_qwen_tools_format

### from_qwen_response

*Rust Method* — `src/llm/provider/qwen.rs#L978-L1216`

_private_

```
fn from_qwen_response(&self, response: QwenResponse, known_tools: &[String]) -> LLMResponse
```

**Calls:** next, is_empty, extract_thinking, parse_hermes_tool_calls, find_after, parse_fallback_tool_calls, push_fallback_or_text, parse_native_qwen_tool_calls, len, clean_incomplete_markers, from_str

**Called by:** complete, stream, stream_events_from_buffered_content, test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display, test_from_qwen_response_stray_closing_tag_before_real_call_does_not_loop_forever, test_from_qwen_response_uses_fallback_when_no_hermes_tags, test_from_qwen_response_openai_parser_still_detects_fallback_json, test_from_qwen_response_detects_bare_json_call_mixed_with_hermes_call

### generate_call_id

*Rust Method* — `src/llm/provider/qwen.rs#L232-L237`

_private_

```
fn generate_call_id() -> String
```

**Called by:** parse_hermes_tool_calls, parse_fallback_tool_calls, parse_native_qwen_tool_calls

### handle_error

*Rust Method* — `src/llm/provider/qwen.rs#L1219-L1269`

_private_

```
async fn handle_error(&self, response: reqwest::Response) -> ProviderError
```

### headers

*Rust Method* — `src/llm/provider/qwen.rs#L246-L267`

_private_

```
fn headers(&self) -> Result<reqwest::header::HeaderMap>
```

**Calls:** is_local, parse

### is_local

*Rust Method* — `src/llm/provider/qwen.rs#L226-L228`

_private_

```
fn is_local(&self) -> bool
```

**Called by:** headers, to_qwen_request, validate_model, calculate_cost

### local

*Rust Method* — `src/llm/provider/qwen.rs#L133-L145`

```
pub fn local(base_url: String) -> Self
```

**Calls:** build_client

### local_only

*Rust Method* — `src/llm/provider/qwen.rs#L902-L908`

_private_

```
fn local_only<T>(is_local: bool, value: Option<T>) -> Option<T>
```

**Called by:** to_qwen_request

### parse_fallback_tool_calls

*Rust Method* — `src/llm/provider/qwen.rs#L439-L478`

_private_

```
fn parse_fallback_tool_calls( &self, text: &str, known_tools: &[String], ) -> (Vec<(String, String, serde_json::Value)>, String)
```

**Calls:** find_json_objects, from_str, generate_call_id, expand_span_over_adjacent_fences, is_empty

**Called by:** push_fallback_or_text, from_qwen_response, test_fallback_parses_bare_json_tool_call, test_fallback_rejects_unregistered_tool_name, test_fallback_parses_fenced_json_tool_call, test_fallback_does_not_corrupt_unrelated_fenced_code_block, test_fallback_ignores_unrelated_json

### parse_hermes_tool_calls

*Rust Method* — `src/llm/provider/qwen.rs#L294-L341`

_private_

```
fn parse_hermes_tool_calls(&self, text: &str) -> Vec<(String, String, serde_json::Value)>
```

**Calls:** find_after, generate_call_id

**Called by:** from_qwen_response, test_hermes_tool_call_parsing, test_multiple_hermes_tool_calls, test_hermes_malformed_json_is_skipped_without_panicking, test_hermes_json_missing_required_fields_is_skipped

### parse_native_qwen_tool_calls

*Rust Method* — `src/llm/provider/qwen.rs#L537-L593`

_private_

```
fn parse_native_qwen_tool_calls(&self, text: &str) -> Vec<(String, String, serde_json::Value)>
```

**Calls:** skip, len, is_empty, generate_call_id

**Called by:** from_qwen_response, test_native_qwen_tool_call_parsing, test_multiple_native_qwen_tool_calls

### calculate_cost

*Rust Method* — `src/llm/provider/qwen.rs#L1586-L1614`

_private_

```
fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64
```

**Calls:** is_local

### complete

*Rust Method* — `src/llm/provider/qwen.rs#L1274-L1323`

_private_

```
async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>
```

**Calls:** to_qwen_request, len, retry_with_backoff, is_success, from_qwen_response

### context_window

*Rust Method* — `src/llm/provider/qwen.rs#L1559-L1584`

_private_

```
fn context_window(&self, model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/llm/provider/qwen.rs#L1521-L1523`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/llm/provider/qwen.rs#L1517-L1519`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/llm/provider/qwen.rs#L1325-L1502`

_private_

```
async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>
```

**Calls:** to_qwen_request, retry_with_backoff, is_success, next, is_empty, len, from_qwen_response, llm_response_to_stream_events

### supported_models

*Rust Method* — `src/llm/provider/qwen.rs#L1525-L1549`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### supports_streaming

*Rust Method* — `src/llm/provider/qwen.rs#L1504-L1506`

_private_

```
fn supports_streaming(&self) -> bool
```

### supports_tools

*Rust Method* — `src/llm/provider/qwen.rs#L1508-L1510`

_private_

```
fn supports_tools(&self) -> bool
```

### supports_vision

*Rust Method* — `src/llm/provider/qwen.rs#L1512-L1515`

_private_

```
fn supports_vision(&self) -> bool
```

### validate_model

*Rust Method* — `src/llm/provider/qwen.rs#L1551-L1557`

_private_

```
fn validate_model(&self, model: &str) -> bool
```

**Calls:** is_local

### push_fallback_or_text

*Rust Method* — `src/llm/provider/qwen.rs#L944-L971`

_private_

```
fn push_fallback_or_text( &self, remaining: String, known_tools: &[String], has_tool_calls: &mut bool, content_blocks: &mut Vec<ContentBlock>, )
```

**Calls:** parse_fallback_tool_calls, is_empty

**Called by:** from_qwen_response

### to_qwen_request

*Rust Method* — `src/llm/provider/qwen.rs#L623-L896`

_private_

```
fn to_qwen_request(&self, request: LLMRequest) -> QwenRequest
```

**Calls:** is_empty, format_hermes_tools, format_native_qwen_tools, format_native_qwen_result, is_local, default_sampling, local_only

**Called by:** complete, stream, test_sampling_defaults_qwen25_coder_local, test_sampling_defaults_qwen3_non_thinking, test_sampling_defaults_qwen3_thinking, test_sampling_defaults_dashscope_omits_vendor_extensions, test_sampling_explicit_request_top_p_wins, test_sampling_config_override_wins_over_defaults, test_sampling_defaults_unrecognized_model_name_is_conservative

### tool_parser

*Rust Method* — `src/llm/provider/qwen.rs#L177-L179`

```
pub(crate) fn tool_parser(&self) -> ToolCallParser
```

### with_base_url

*Rust Method* — `src/llm/provider/qwen.rs#L148-L160`

```
pub fn with_base_url(api_key: String, base_url: String) -> Self
```

**Calls:** build_client

### with_default_model

*Rust Method* — `src/llm/provider/qwen.rs#L163-L166`

```
pub fn with_default_model(mut self, model: String) -> Self
```

### with_sampling

*Rust Method* — `src/llm/provider/qwen.rs#L187-L199`

```
pub fn with_sampling( mut self, top_p: Option<f32>, top_k: Option<u32>, repetition_penalty: Option<f32>, ) -> Self
```

### with_thinking

*Rust Method* — `src/llm/provider/qwen.rs#L202-L205`

```
pub fn with_thinking(mut self, enabled: bool) -> Self
```

### with_thinking_budget

*Rust Method* — `src/llm/provider/qwen.rs#L208-L211`

```
pub fn with_thinking_budget(mut self, budget_tokens: u32) -> Self
```

**Called by:** configure_qwen, test_thinking_mode_configuration

### with_tool_parser

*Rust Method* — `src/llm/provider/qwen.rs#L169-L172`

```
pub fn with_tool_parser(mut self, parser: ToolCallParser) -> Self
```

**Called by:** configure_qwen, test_tool_parser_configuration, test_native_qwen_parser_configuration, test_native_qwen_tool_call_parsing, test_multiple_native_qwen_tool_calls, test_native_qwen_tools_format, test_native_qwen_result_format, test_clean_incomplete_markers, test_from_qwen_response_openai_parser_still_detects_fallback_json

### find_after

*Rust Function* — `src/llm/provider/qwen.rs#L105-L107`

_private_

```
fn find_after(haystack: &str, start: usize, needle: &str) -> Option<usize>
```

**Called by:** parse_hermes_tool_calls, extract_thinking, from_qwen_response, find_after_returns_an_absolute_offset_not_a_relative_one

### find_after_ignores_a_match_before_start

*Rust Function* — `src/llm/provider/qwen.rs#L1854-L1861`

_private_

```
fn find_after_ignores_a_match_before_start()
```

### find_after_returns_an_absolute_offset_not_a_relative_one

*Rust Function* — `src/llm/provider/qwen.rs#L1871-L1876`

_private_

```
fn find_after_returns_an_absolute_offset_not_a_relative_one()
```

**Calls:** find_after

### find_after_returns_none_when_nothing_matches_after_start

*Rust Function* — `src/llm/provider/qwen.rs#L1864-L1868`

_private_

```
fn find_after_returns_none_when_nothing_matches_after_start()
```

### llm_response_to_stream_events

*Rust Function* — `src/llm/provider/qwen.rs#L1624-L1697`

_private_

```
fn llm_response_to_stream_events(response: LLMResponse) -> Vec<StreamEvent>
```

**Calls:** is_empty

**Called by:** stream, stream_events_from_buffered_content

### mock_sse_server

*Rust Function* — `src/llm/provider/qwen.rs#L1968-L2004`

_private_

```
async fn mock_sse_server(body: String) -> String
```

**Called by:** stream_assembles_openai_style_tool_call_across_sse_chunks, stream_skips_malformed_sse_chunk_and_continues

### stream_assembles_openai_style_tool_call_across_sse_chunks

*Rust Function* — `src/llm/provider/qwen.rs#L2012-L2055`

_private_

```
async fn stream_assembles_openai_style_tool_call_across_sse_chunks()
```

**Calls:** mock_sse_server

### stream_events_from_buffered_content

*Rust Function* — `src/llm/provider/qwen.rs#L1882-L1907`

_private_

```
fn stream_events_from_buffered_content( provider: &QwenProvider, content: &str, known_tools: &[String], ) -> Vec<StreamEvent>
```

**Calls:** from_qwen_response, llm_response_to_stream_events

**Called by:** streaming_assembles_hermes_tool_call_from_buffered_text, streaming_plain_text_roundtrips_without_tool_calls

### stream_skips_malformed_sse_chunk_and_continues

*Rust Function* — `src/llm/provider/qwen.rs#L2060-L2090`

_private_

```
async fn stream_skips_malformed_sse_chunk_and_continues()
```

**Calls:** mock_sse_server

### streaming_assembles_hermes_tool_call_from_buffered_text

*Rust Function* — `src/llm/provider/qwen.rs#L1912-L1936`

_private_

```
fn streaming_assembles_hermes_tool_call_from_buffered_text()
```

**Calls:** stream_events_from_buffered_content

### streaming_plain_text_roundtrips_without_tool_calls

*Rust Function* — `src/llm/provider/qwen.rs#L1940-L1962`

_private_

```
fn streaming_plain_text_roundtrips_without_tool_calls()
```

**Calls:** stream_events_from_buffered_content

### test_calculate_cost_cloud

*Rust Function* — `src/llm/provider/qwen.rs#L2360-L2365`

_private_

```
fn test_calculate_cost_cloud()
```

**Calls:** dashscope_intl

### test_calculate_cost_local

*Rust Function* — `src/llm/provider/qwen.rs#L2353-L2357`

_private_

```
fn test_calculate_cost_local()
```

### test_calculate_cost_unknown_cloud_model_returns_zero

*Rust Function* — `src/llm/provider/qwen.rs#L2368-L2371`

_private_

```
fn test_calculate_cost_unknown_cloud_model_returns_zero()
```

**Calls:** dashscope_intl

### test_clean_incomplete_markers

*Rust Function* — `src/llm/provider/qwen.rs#L2570-L2583`

_private_

```
fn test_clean_incomplete_markers()
```

**Calls:** with_tool_parser, clean_incomplete_markers

### test_context_window

*Rust Function* — `src/llm/provider/qwen.rs#L2344-L2350`

_private_

```
fn test_context_window()
```

**Calls:** dashscope_intl

### test_custom_default_model

*Rust Function* — `src/llm/provider/qwen.rs#L2382-L2386`

_private_

```
fn test_custom_default_model()
```

### test_fallback_does_not_corrupt_unrelated_fenced_code_block

*Rust Function* — `src/llm/provider/qwen.rs#L2638-L2650`

_private_

```
fn test_fallback_does_not_corrupt_unrelated_fenced_code_block()
```

**Calls:** parse_fallback_tool_calls

### test_fallback_ignores_unrelated_json

*Rust Function* — `src/llm/provider/qwen.rs#L2655-L2663`

_private_

```
fn test_fallback_ignores_unrelated_json()
```

**Calls:** parse_fallback_tool_calls

### test_fallback_parses_bare_json_tool_call

*Rust Function* — `src/llm/provider/qwen.rs#L2588-L2600`

_private_

```
fn test_fallback_parses_bare_json_tool_call()
```

**Calls:** parse_fallback_tool_calls

### test_fallback_parses_fenced_json_tool_call

*Rust Function* — `src/llm/provider/qwen.rs#L2621-L2632`

_private_

```
fn test_fallback_parses_fenced_json_tool_call()
```

**Calls:** parse_fallback_tool_calls

### test_fallback_rejects_unregistered_tool_name

*Rust Function* — `src/llm/provider/qwen.rs#L2607-L2616`

_private_

```
fn test_fallback_rejects_unregistered_tool_name()
```

**Calls:** parse_fallback_tool_calls

### test_find_json_objects_recovers_nested_object_after_failed_outer_parse

*Rust Function* — `src/llm/provider/qwen.rs#L2669-L2680`

_private_

```
fn test_find_json_objects_recovers_nested_object_after_failed_outer_parse()
```

**Calls:** find_json_objects

### test_from_qwen_response_detects_bare_json_call_mixed_with_hermes_call

*Rust Function* — `src/llm/provider/qwen.rs#L2770-L2807`

_private_

```
fn test_from_qwen_response_detects_bare_json_call_mixed_with_hermes_call()
```

**Calls:** from_qwen_response

### test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display

*Rust Function* — `src/llm/provider/qwen.rs#L2191-L2246`

_private_

```
fn test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display()
```

**Calls:** from_qwen_response

### test_from_qwen_response_openai_parser_still_detects_fallback_json

*Rust Function* — `src/llm/provider/qwen.rs#L2728-L2763`

_private_

```
fn test_from_qwen_response_openai_parser_still_detects_fallback_json()
```

**Calls:** with_tool_parser, from_qwen_response

### test_from_qwen_response_stray_closing_tag_before_real_call_does_not_loop_forever

*Rust Function* — `src/llm/provider/qwen.rs#L2256-L2295`

_private_

```
fn test_from_qwen_response_stray_closing_tag_before_real_call_does_not_loop_forever()
```

**Calls:** from_qwen_response

### test_from_qwen_response_uses_fallback_when_no_hermes_tags

*Rust Function* — `src/llm/provider/qwen.rs#L2686-L2721`

_private_

```
fn test_from_qwen_response_uses_fallback_when_no_hermes_tags()
```

**Calls:** from_qwen_response

### test_hermes_json_missing_required_fields_is_skipped

*Rust Function* — `src/llm/provider/qwen.rs#L2173-L2182`

_private_

```
fn test_hermes_json_missing_required_fields_is_skipped()
```

**Calls:** parse_hermes_tool_calls

### test_hermes_malformed_json_is_skipped_without_panicking

*Rust Function* — `src/llm/provider/qwen.rs#L2158-L2168`

_private_

```
fn test_hermes_malformed_json_is_skipped_without_panicking()
```

**Calls:** parse_hermes_tool_calls

### test_hermes_tool_call_parsing

*Rust Function* — `src/llm/provider/qwen.rs#L2123-L2135`

_private_

```
fn test_hermes_tool_call_parsing()
```

**Calls:** parse_hermes_tool_calls

### test_hermes_tools_format

*Rust Function* — `src/llm/provider/qwen.rs#L2472-L2492`

_private_

```
fn test_hermes_tools_format()
```

**Calls:** format_hermes_tools

### test_local_provider_creation

*Rust Function* — `src/llm/provider/qwen.rs#L2100-L2104`

_private_

```
fn test_local_provider_creation()
```

### test_multiple_hermes_tool_calls

*Rust Function* — `src/llm/provider/qwen.rs#L2138-L2153`

_private_

```
fn test_multiple_hermes_tool_calls()
```

**Calls:** parse_hermes_tool_calls

### test_multiple_native_qwen_tool_calls

*Rust Function* — `src/llm/provider/qwen.rs#L2518-L2531`

_private_

```
fn test_multiple_native_qwen_tool_calls()
```

**Calls:** with_tool_parser, parse_native_qwen_tool_calls

### test_native_qwen_parser_configuration

*Rust Function* — `src/llm/provider/qwen.rs#L2495-L2499`

_private_

```
fn test_native_qwen_parser_configuration()
```

**Calls:** with_tool_parser

### test_native_qwen_result_format

*Rust Function* — `src/llm/provider/qwen.rs#L2559-L2567`

_private_

```
fn test_native_qwen_result_format()
```

**Calls:** with_tool_parser, format_native_qwen_result

### test_native_qwen_tool_call_parsing

*Rust Function* — `src/llm/provider/qwen.rs#L2502-L2515`

_private_

```
fn test_native_qwen_tool_call_parsing()
```

**Calls:** with_tool_parser, parse_native_qwen_tool_calls

### test_native_qwen_tools_format

*Rust Function* — `src/llm/provider/qwen.rs#L2534-L2556`

_private_

```
fn test_native_qwen_tools_format()
```

**Calls:** with_tool_parser, format_native_qwen_tools

### test_qwen_provider_creation

*Rust Function* — `src/llm/provider/qwen.rs#L2093-L2097`

_private_

```
fn test_qwen_provider_creation()
```

**Calls:** dashscope_intl

### test_sampling_config_override_wins_over_defaults

*Rust Function* — `src/llm/provider/qwen.rs#L2457-L2469`

_private_

```
fn test_sampling_config_override_wins_over_defaults()
```

**Calls:** dashscope_intl, to_qwen_request

### test_sampling_defaults_dashscope_omits_vendor_extensions

*Rust Function* — `src/llm/provider/qwen.rs#L2431-L2439`

_private_

```
fn test_sampling_defaults_dashscope_omits_vendor_extensions()
```

**Calls:** dashscope_intl, to_qwen_request

### test_sampling_defaults_qwen25_coder_local

*Rust Function* — `src/llm/provider/qwen.rs#L2392-L2400`

_private_

```
fn test_sampling_defaults_qwen25_coder_local()
```

**Calls:** to_qwen_request

### test_sampling_defaults_qwen3_non_thinking

*Rust Function* — `src/llm/provider/qwen.rs#L2405-L2413`

_private_

```
fn test_sampling_defaults_qwen3_non_thinking()
```

**Calls:** to_qwen_request

### test_sampling_defaults_qwen3_thinking

*Rust Function* — `src/llm/provider/qwen.rs#L2417-L2425`

_private_

```
fn test_sampling_defaults_qwen3_thinking()
```

**Calls:** to_qwen_request

### test_sampling_defaults_unrecognized_model_name_is_conservative

*Rust Function* — `src/llm/provider/qwen.rs#L2814-L2822`

_private_

```
fn test_sampling_defaults_unrecognized_model_name_is_conservative()
```

**Calls:** to_qwen_request

### test_sampling_explicit_request_top_p_wins

*Rust Function* — `src/llm/provider/qwen.rs#L2444-L2451`

_private_

```
fn test_sampling_explicit_request_top_p_wins()
```

**Calls:** with_top_p, to_qwen_request

### test_stop_words_defined

*Rust Function* — `src/llm/provider/qwen.rs#L2825-L2830`

_private_

```
fn test_stop_words_defined()
```

### test_supported_models

*Rust Function* — `src/llm/provider/qwen.rs#L2333-L2341`

_private_

```
fn test_supported_models()
```

**Calls:** dashscope_intl

### test_thinking_extraction

*Rust Function* — `src/llm/provider/qwen.rs#L2298-L2312`

_private_

```
fn test_thinking_extraction()
```

**Calls:** extract_thinking

### test_thinking_extraction_out_of_order_tags_does_not_panic

*Rust Function* — `src/llm/provider/qwen.rs#L2320-L2330`

_private_

```
fn test_thinking_extraction_out_of_order_tags_does_not_panic()
```

**Calls:** extract_thinking

### test_thinking_mode_configuration

*Rust Function* — `src/llm/provider/qwen.rs#L2114-L2120`

_private_

```
fn test_thinking_mode_configuration()
```

**Calls:** with_thinking_budget

### test_tool_call_index_in_bounds

*Rust Function* — `src/llm/provider/qwen.rs#L2374-L2379`

_private_

```
fn test_tool_call_index_in_bounds()
```

### test_tool_parser_configuration

*Rust Function* — `src/llm/provider/qwen.rs#L2107-L2111`

_private_

```
fn test_tool_parser_configuration()
```

**Calls:** with_tool_parser

### tool_call_index_in_bounds

*Rust Function* — `src/llm/provider/qwen.rs#L48-L50`

_private_

```
fn tool_call_index_in_bounds(idx: usize) -> bool
```

### aggressive

*Rust Method* — `src/llm/provider/retry.rs#L62-L70`

```
pub fn aggressive() -> Self
```

### calculate_delay

*Rust Method* — `src/llm/provider/retry.rs#L73-L92`

_private_

```
fn calculate_delay(&self, attempt: u32) -> Duration
```

### default

*Rust Method* — `src/llm/provider/retry.rs#L32-L40`

_private_

```
fn default() -> Self
```

### new

*Rust Method* — `src/llm/provider/retry.rs#L45-L51`

```
pub fn new(max_attempts: u32, initial_delay: Duration) -> Self
```

### no_retry

*Rust Method* — `src/llm/provider/retry.rs#L54-L59`

```
pub fn no_retry() -> Self
```

### extract_retry_after

*Rust Function* — `src/llm/provider/retry.rs#L196-L216`

```
pub fn extract_retry_after(error: &ProviderError) -> Option<Duration>
```

**Calls:** parse_retry_seconds

**Called by:** test_extract_retry_after

### parse_retry_seconds

*Rust Function* — `src/llm/provider/retry.rs#L219-L244`

_private_

```
fn parse_retry_seconds(msg: &str) -> Option<u64>
```

**Called by:** extract_retry_after

### retry_with_backoff

*Rust Function* — `src/llm/provider/retry.rs#L112-L166`

```
pub async fn retry_with_backoff<F, Fut, T>(mut operation: F, config: &RetryConfig) -> Result<T> where F: FnMut() -> Fut, Fut: Future<Output = Result<T>>,
```

**Called by:** complete, stream, complete, stream, complete, stream, complete, stream, retry_with_rate_limit, test_retry_success_immediate, test_retry_success_after_retries, test_retry_max_attempts_exceeded, test_retry_non_retryable_error

### retry_with_rate_limit

*Rust Function* — `src/llm/provider/retry.rs#L171-L191`

```
pub async fn retry_with_rate_limit<F, Fut, T>( operation: F, config: &RetryConfig, retry_after: Option<Duration>, ) -> Result<T> where F: FnMut() -> Fut, Fut: Future<Output = Result<T>>,
```

**Calls:** retry_with_backoff

### test_calculate_delay

*Rust Function* — `src/llm/provider/retry.rs#L265-L289`

_private_

```
fn test_calculate_delay()
```

### test_extract_retry_after

*Rust Function* — `src/llm/provider/retry.rs#L398-L412`

_private_

```
fn test_extract_retry_after()
```

**Calls:** extract_retry_after

### test_parse_retry_seconds

*Rust Function* — `src/llm/provider/retry.rs#L415-L420`

_private_

```
fn test_parse_retry_seconds()
```

### test_retry_config_defaults

*Rust Function* — `src/llm/provider/retry.rs#L251-L256`

_private_

```
fn test_retry_config_defaults()
```

### test_retry_config_no_retry

*Rust Function* — `src/llm/provider/retry.rs#L259-L262`

_private_

```
fn test_retry_config_no_retry()
```

### test_retry_max_attempts_exceeded

*Rust Function* — `src/llm/provider/retry.rs#L348-L370`

_private_

```
async fn test_retry_max_attempts_exceeded()
```

**Calls:** retry_with_backoff

### test_retry_non_retryable_error

*Rust Function* — `src/llm/provider/retry.rs#L373-L395`

_private_

```
async fn test_retry_non_retryable_error()
```

**Calls:** retry_with_backoff

### test_retry_success_after_retries

*Rust Function* — `src/llm/provider/retry.rs#L318-L345`

_private_

```
async fn test_retry_success_after_retries()
```

**Calls:** retry_with_backoff

### test_retry_success_immediate

*Rust Function* — `src/llm/provider/retry.rs#L292-L315`

_private_

```
async fn test_retry_success_immediate()
```

**Calls:** retry_with_backoff

### context_window

*Rust Method* — `src/llm/provider/router.rs#L60-L65`

```
pub fn context_window(&self, tier: ModelTier) -> u32
```

### default

*Rust Method* — `src/llm/provider/router.rs#L103-L105`

_private_

```
fn default() -> Self
```

**Calls:** default_anthropic

### default_anthropic

*Rust Method* — `src/llm/provider/router.rs#L77-L86`

```
pub fn default_anthropic() -> Self
```

**Called by:** default

### default_for_test

*Rust Method* — `src/llm/provider/router.rs#L90-L99`

```
pub fn default_for_test() -> Self
```

**Called by:** all_tiers_resolve_to_non_empty_model, token_limits_ordered_correctly, thinking_budget_by_tier

### max_output_tokens

*Rust Method* — `src/llm/provider/router.rs#L51-L57`

```
pub fn max_output_tokens(&self, tier: ModelTier) -> u32
```

### new

*Rust Method* — `src/llm/provider/router.rs#L26-L39`

```
pub fn new( fast_provider: &str, fast_model: &str, balanced_provider: &str, balanced_model: &str, powerful_provider: &str, powerful_model: &str, ) -> Self
```

### resolve

*Rust Method* — `src/llm/provider/router.rs#L42-L48`

```
pub fn resolve(&self, tier: ModelTier) -> (&str, &str)
```

### thinking_budget

*Rust Method* — `src/llm/provider/router.rs#L68-L74`

```
pub fn thinking_budget(&self, tier: ModelTier) -> Option<u32>
```

### all_tiers_resolve_to_non_empty_model

*Rust Function* — `src/llm/provider/router.rs#L113-L120`

_private_

```
fn all_tiers_resolve_to_non_empty_model()
```

**Calls:** default_for_test

### thinking_budget_by_tier

*Rust Function* — `src/llm/provider/router.rs#L138-L143`

_private_

```
fn thinking_budget_by_tier()
```

**Calls:** default_for_test

### token_limits_ordered_correctly

*Rust Function* — `src/llm/provider/router.rs#L123-L135`

_private_

```
fn token_limits_ordered_correctly()
```

**Calls:** default_for_test

### calculate_cost

*Rust Method* — `src/llm/provider/trait.rs#L121-L123`

_private_

```
fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64
```

### complete

*Rust Method* — `src/llm/provider/trait.rs#L97-L99`

_private_

```
async fn complete(&self, _request: LLMRequest) -> Result<LLMResponse>
```

### context_window

*Rust Method* — `src/llm/provider/trait.rs#L117-L119`

_private_

```
fn context_window(&self, _model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/llm/provider/trait.rs#L109-L111`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/llm/provider/trait.rs#L105-L107`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/llm/provider/trait.rs#L101-L103`

_private_

```
async fn stream(&self, _request: LLMRequest) -> Result<ProviderStream>
```

### supported_models

*Rust Method* — `src/llm/provider/trait.rs#L113-L115`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### supports_streaming

*Rust Method* — `src/llm/provider/trait.rs#L32-L34`

_private_

```
fn supports_streaming(&self) -> bool
```

### supports_tools

*Rust Method* — `src/llm/provider/trait.rs#L37-L39`

_private_

```
fn supports_tools(&self) -> bool
```

### supports_vision

*Rust Method* — `src/llm/provider/trait.rs#L42-L44`

_private_

```
fn supports_vision(&self) -> bool
```

### validate_model

*Rust Method* — `src/llm/provider/trait.rs#L56-L58`

_private_

```
fn validate_model(&self, model: &str) -> bool
```

### for_provider

*Rust Method* — `src/llm/provider/trait.rs#L78-L85`

```
pub fn for_provider(provider: &dyn Provider) -> Self
```

**Called by:** test_provider_capabilities

### test_provider_capabilities

*Rust Function* — `src/llm/provider/trait.rs#L135-L141`

_private_

```
fn test_provider_capabilities()
```

**Calls:** for_provider

### test_provider_validate_model

*Rust Function* — `src/llm/provider/trait.rs#L127-L132`

_private_

```
fn test_provider_validate_model()
```

### hit_rate

*Rust Method* — `src/llm/provider/types.rs#L281-L288`

```
pub fn hit_rate(&self) -> f32
```

### new

*Rust Method* — `src/llm/provider/types.rs#L157-L175`

```
pub fn new(model: impl Into<String>, messages: Vec<Message>) -> Self
```

### with_frequency_penalty

*Rust Method* — `src/llm/provider/types.rs#L228-L231`

```
pub fn with_frequency_penalty(mut self, penalty: f32) -> Self
```

**Called by:** test_llm_request_new_fields, test_new_fields_forwarded_to_openai_request

### with_max_tokens

*Rust Method* — `src/llm/provider/types.rs#L247-L250`

```
pub fn with_max_tokens(mut self, max_tokens: u32) -> Self
```

**Called by:** send_message_with_tools_inner, prepare_message_context, streamed_ollama_tool_call_survives_drain, test_to_ollama_request_maps_common_fields, test_llm_request_builder

### with_presence_penalty

*Rust Method* — `src/llm/provider/types.rs#L234-L237`

```
pub fn with_presence_penalty(mut self, penalty: f32) -> Self
```

**Called by:** test_llm_request_new_fields, test_new_fields_forwarded_to_openai_request

### with_response_format

*Rust Method* — `src/llm/provider/types.rs#L241-L244`

```
pub fn with_response_format(mut self, format: serde_json::Value) -> Self
```

**Called by:** test_json_mode_sets_response_mime_type, test_full_json_schema_sets_response_schema, to_ollama_request_maps_thinking_and_response_format, test_llm_request_new_fields, test_new_fields_forwarded_to_openai_request

### with_seed

*Rust Method* — `src/llm/provider/types.rs#L216-L219`

```
pub fn with_seed(mut self, seed: u64) -> Self
```

**Called by:** test_to_ollama_request_maps_common_fields, test_llm_request_new_fields, test_new_fields_forwarded_to_openai_request

### with_stop

*Rust Method* — `src/llm/provider/types.rs#L222-L225`

```
pub fn with_stop(mut self, stop: Vec<String>) -> Self
```

**Called by:** test_to_ollama_request_maps_common_fields, test_llm_request_new_fields, test_new_fields_forwarded_to_openai_request

### with_streaming

*Rust Method* — `src/llm/provider/types.rs#L253-L256`

```
pub fn with_streaming(mut self) -> Self
```

**Called by:** send_message_streaming, call_provider_streaming, streamed_ollama_tool_call_survives_drain, test_llm_request_builder, test_streaming_basic, test_streaming_single_chunk, test_streaming_multiple_chunks, test_streaming_token_counting, test_streaming_stop_reason, test_streaming_error_handling, test_streaming_empty_response, test_streaming_content_accumulation, test_streaming_request_builder

### with_system

*Rust Method* — `src/llm/provider/types.rs#L191-L194`

```
pub fn with_system(mut self, system: impl Into<String>) -> Self
```

**Called by:** send_message_with_tools_inner, prepare_message_context, streamed_ollama_tool_call_survives_drain, test_to_gemini_request_maps_system_and_tools, test_to_ollama_request_maps_common_fields, test_llm_request_builder

### with_temperature

*Rust Method* — `src/llm/provider/types.rs#L203-L206`

```
pub fn with_temperature(mut self, temperature: f32) -> Self
```

**Called by:** test_to_ollama_request_maps_common_fields, test_llm_request_builder

### with_thinking

*Rust Method* — `src/llm/provider/types.rs#L179-L188`

```
pub fn with_thinking(mut self, budget_tokens: u32) -> Self
```

### with_tools

*Rust Method* — `src/llm/provider/types.rs#L197-L200`

```
pub fn with_tools(mut self, tools: Vec<Tool>) -> Self
```

**Called by:** send_message_with_tools_inner, streamed_ollama_tool_call_survives_drain, test_to_gemini_request_maps_system_and_tools, streamed_tool_call_reaches_caller

### with_top_p

*Rust Method* — `src/llm/provider/types.rs#L210-L213`

```
pub fn with_top_p(mut self, top_p: f32) -> Self
```

**Called by:** test_to_ollama_request_maps_common_fields, test_llm_request_new_fields, test_new_fields_forwarded_to_openai_request, test_sampling_explicit_request_top_p_wins

### assistant

*Rust Method* — `src/llm/provider/types.rs#L39-L44`

```
pub fn assistant(text: impl Into<String>) -> Self
```

**Called by:** recovered_tool_call_becomes_a_tool_use_block, fenced_call_in_prose_becomes_a_tool_use_block, from_ollama_response_plain_text_with_final_data, from_ollama_response_without_final_data_has_zero_usage_and_no_perf, from_ollama_response_extracts_tool_calls, from_ollama_response_uses_explicit_thinking_field, from_ollama_response_falls_back_to_think_tags, test_message_creation

### system

*Rust Method* — `src/llm/provider/types.rs#L47-L52`

```
pub fn system(text: impl Into<String>) -> Self
```

**Called by:** to_ollama_request

### user

*Rust Method* — `src/llm/provider/types.rs#L31-L36`

```
pub fn user(text: impl Into<String>) -> Self
```

**Called by:** test_add_message, test_would_exceed_limit, test_usage_percentage, test_trim_to_fit, send_message_with_tools_inner, prepare_message_context, test_message_creation

### tokens_per_second

*Rust Method* — `src/llm/provider/types.rs#L335-L338`

```
pub fn tokens_per_second(&self, output_tokens: u32) -> Option<f64>
```

**Called by:** complete_response

### total

*Rust Method* — `src/llm/provider/types.rs#L366-L368`

```
pub fn total(&self) -> u32
```

### cache_metrics_hit_rate

*Rust Function* — `src/llm/provider/types.rs#L540-L548`

_private_

```
fn cache_metrics_hit_rate()
```

### extract_think_tags

*Rust Function* — `src/llm/provider/types.rs#L439-L472`

```
pub fn extract_think_tags(text: &str) -> (String, String)
```

**Calls:** len, is_empty

**Called by:** drain_stream_to_response, from_ollama_response, from_openai_response, extract_think_tags_single_block, extract_think_tags_multiple_blocks, extract_think_tags_no_tags, extract_think_tags_unclosed, extract_think_tags_only_block

### extract_think_tags_multiple_blocks

*Rust Function* — `src/llm/provider/types.rs#L583-L589`

_private_

```
fn extract_think_tags_multiple_blocks()
```

**Calls:** extract_think_tags

### extract_think_tags_no_tags

*Rust Function* — `src/llm/provider/types.rs#L592-L596`

_private_

```
fn extract_think_tags_no_tags()
```

**Calls:** extract_think_tags

### extract_think_tags_only_block

*Rust Function* — `src/llm/provider/types.rs#L606-L610`

_private_

```
fn extract_think_tags_only_block()
```

**Calls:** extract_think_tags

### extract_think_tags_single_block

*Rust Function* — `src/llm/provider/types.rs#L575-L580`

_private_

```
fn extract_think_tags_single_block()
```

**Calls:** extract_think_tags

### extract_think_tags_unclosed

*Rust Function* — `src/llm/provider/types.rs#L599-L603`

_private_

```
fn extract_think_tags_unclosed()
```

**Calls:** extract_think_tags

### perf_metrics_tokens_per_second

*Rust Function* — `src/llm/provider/types.rs#L551-L557`

_private_

```
fn perf_metrics_tokens_per_second()
```

### perf_metrics_tokens_per_second_missing_duration

*Rust Function* — `src/llm/provider/types.rs#L560-L563`

_private_

```
fn perf_metrics_tokens_per_second_missing_duration()
```

### perf_metrics_tokens_per_second_zero_duration

*Rust Function* — `src/llm/provider/types.rs#L566-L572`

_private_

```
fn perf_metrics_tokens_per_second_zero_duration()
```

### test_llm_request_builder

*Rust Function* — `src/llm/provider/types.rs#L489-L501`

_private_

```
fn test_llm_request_builder()
```

**Calls:** with_system, with_temperature, with_max_tokens, with_streaming

### test_message_creation

*Rust Function* — `src/llm/provider/types.rs#L479-L486`

_private_

```
fn test_message_creation()
```

**Calls:** user, assistant

### test_token_usage

*Rust Function* — `src/llm/provider/types.rs#L504-L510`

_private_

```
fn test_token_usage()
```

### with_thinking_sets_temperature

*Rust Function* — `src/llm/provider/types.rs#L522-L528`

_private_

```
fn with_thinking_sets_temperature()
```

### with_thinking_sets_temperature_and_config

*Rust Function* — `src/llm/provider/types.rs#L513-L519`

_private_

```
fn with_thinking_sets_temperature_and_config()
```

### with_thinking_zero_budget_is_noop

*Rust Function* — `src/llm/provider/types.rs#L531-L537`

_private_

```
fn with_thinking_zero_budget_is_noop()
```

### capabilities

*Rust Method* — `src/llm/tools/agent.rs#L90-L92`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/agent.rs#L57-L62`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/agent.rs#L115-L202`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** is_empty, slugify, launch, with_metadata

### input_schema

*Rust Method* — `src/llm/tools/agent.rs#L64-L88`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/agent.rs#L53-L55`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/agent.rs#L94-L96`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/agent.rs#L98-L113`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** is_empty

### slugify

*Rust Function* — `src/llm/tools/agent.rs#L206-L220`

_private_

```
fn slugify(s: &str) -> String
```

**Calls:** is_empty

**Called by:** execute

### test_slugify

*Rust Function* — `src/llm/tools/agent.rs#L227-L231`

_private_

```
fn test_slugify()
```

### test_validate_empty_description

*Rust Function* — `src/llm/tools/agent.rs#L234-L239`

_private_

```
fn test_validate_empty_description()
```

### test_validate_empty_prompt

*Rust Function* — `src/llm/tools/agent.rs#L242-L247`

_private_

```
fn test_validate_empty_prompt()
```

### test_validate_valid_input

*Rust Function* — `src/llm/tools/agent.rs#L250-L257`

_private_

```
fn test_validate_valid_input()
```

### alias_resolution_is_a_single_hop

*Rust Function* — `src/llm/tools/aliases.rs#L122-L132`

_private_

```
fn alias_resolution_is_a_single_hop()
```

### no_duplicate_alias_entries

*Rust Function* — `src/llm/tools/aliases.rs#L135-L141`

_private_

```
fn no_duplicate_alias_entries()
```

### resolution_is_case_insensitive

*Rust Function* — `src/llm/tools/aliases.rs#L105-L109`

_private_

```
fn resolution_is_case_insensitive()
```

### resolve

*Rust Function* — `src/llm/tools/aliases.rs#L83-L88`

```
pub fn resolve(name: &str) -> Option<&'static str>
```

### resolves_known_claude_code_alias

*Rust Function* — `src/llm/tools/aliases.rs#L100-L102`

_private_

```
fn resolves_known_claude_code_alias()
```

### resolves_known_qwen_code_alias

*Rust Function* — `src/llm/tools/aliases.rs#L95-L97`

_private_

```
fn resolves_known_qwen_code_alias()
```

### unknown_name_resolves_to_none

*Rust Function* — `src/llm/tools/aliases.rs#L112-L114`

_private_

```
fn unknown_name_resolves_to_none()
```

### capabilities

*Rust Method* — `src/llm/tools/apply_patch.rs#L324-L330`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/apply_patch.rs#L300-L309`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/apply_patch.rs#L343-L518`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** parse_patch, len, check_path, validate_path_safety, validate_file_path, of, apply_hunks, record

### input_schema

*Rust Method* — `src/llm/tools/apply_patch.rs#L311-L322`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/apply_patch.rs#L296-L298`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/apply_patch.rs#L332-L334`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/apply_patch.rs#L336-L341`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** parse_patch

### apply_hunks

*Rust Function* — `src/llm/tools/apply_patch.rs#L235-L285`

_private_

```
fn apply_hunks(original: &str, hunks: &[Hunk]) -> std::result::Result<String, String>
```

**Calls:** is_empty, len, find_subsequence

**Called by:** execute, apply_hunks_replaces_matched_context, apply_hunks_second_hunk_searches_after_first, apply_hunks_errors_when_context_not_found

### apply_hunks_errors_when_context_not_found

*Rust Function* — `src/llm/tools/apply_patch.rs#L638-L645`

_private_

```
fn apply_hunks_errors_when_context_not_found()
```

**Calls:** apply_hunks

### apply_hunks_replaces_matched_context

*Rust Function* — `src/llm/tools/apply_patch.rs#L600-L612`

_private_

```
fn apply_hunks_replaces_matched_context()
```

**Calls:** apply_hunks

### apply_hunks_second_hunk_searches_after_first

*Rust Function* — `src/llm/tools/apply_patch.rs#L615-L635`

_private_

```
fn apply_hunks_second_hunk_searches_after_first()
```

**Calls:** apply_hunks

### context

*Rust Function* — `src/llm/tools/apply_patch.rs#L527-L530`

_private_

```
fn context(temp_dir: &TempDir) -> ToolExecutionContext
```

### execute_add_and_delete_need_no_prior_read

*Rust Function* — `src/llm/tools/apply_patch.rs#L890-L906`

_private_

```
async fn execute_add_and_delete_need_no_prior_read()
```

### execute_add_file_that_already_exists_fails

*Rust Function* — `src/llm/tools/apply_patch.rs#L687-L708`

_private_

```
async fn execute_add_file_that_already_exists_fails()
```

### execute_adds_a_new_file

*Rust Function* — `src/llm/tools/apply_patch.rs#L671-L684`

_private_

```
async fn execute_adds_a_new_file()
```

### execute_applies_multiple_file_ops_in_one_patch

*Rust Function* — `src/llm/tools/apply_patch.rs#L753-L784`

_private_

```
async fn execute_applies_multiple_file_ops_in_one_patch()
```

### execute_blocked_in_read_only_mode

*Rust Function* — `src/llm/tools/apply_patch.rs#L837-L854`

_private_

```
async fn execute_blocked_in_read_only_mode()
```

### execute_deletes_a_file

*Rust Function* — `src/llm/tools/apply_patch.rs#L711-L724`

_private_

```
async fn execute_deletes_a_file()
```

### execute_is_atomic_across_files_on_failure

*Rust Function* — `src/llm/tools/apply_patch.rs#L792-L834`

_private_

```
async fn execute_is_atomic_across_files_on_failure()
```

**Calls:** record, of

### execute_renames_via_move_to

*Rust Function* — `src/llm/tools/apply_patch.rs#L727-L750`

_private_

```
async fn execute_renames_via_move_to()
```

### execute_update_rejects_a_file_never_read_this_session

*Rust Function* — `src/llm/tools/apply_patch.rs#L866-L886`

_private_

```
async fn execute_update_rejects_a_file_never_read_this_session()
```

### execute_updates_an_existing_file

*Rust Function* — `src/llm/tools/apply_patch.rs#L648-L668`

_private_

```
async fn execute_updates_an_existing_file()
```

### find_subsequence

*Rust Function* — `src/llm/tools/apply_patch.rs#L220-L229`

_private_

```
fn find_subsequence(haystack: &[String], needle: &[&str], start: usize) -> Option<usize>
```

**Calls:** is_empty, len

**Called by:** apply_hunks

### parse_add_file_collects_plus_prefixed_lines

*Rust Function* — `src/llm/tools/apply_patch.rs#L559-L569`

_private_

```
fn parse_add_file_collects_plus_prefixed_lines()
```

**Calls:** parse_patch

### parse_multiple_file_ops_in_one_patch

*Rust Function* — `src/llm/tools/apply_patch.rs#L572-L583`

_private_

```
fn parse_multiple_file_ops_in_one_patch()
```

**Calls:** parse_patch

### parse_patch

*Rust Function* — `src/llm/tools/apply_patch.rs#L92-L215`

_private_

```
fn parse_patch(text: &str) -> std::result::Result<Vec<FileOp>, String>
```

**Calls:** len, is_empty

**Called by:** validate_input, execute, parse_rejects_missing_begin_marker, parse_rejects_missing_end_marker, parse_add_file_collects_plus_prefixed_lines, parse_multiple_file_ops_in_one_patch, parse_update_with_move_to

### parse_rejects_missing_begin_marker

*Rust Function* — `src/llm/tools/apply_patch.rs#L547-L550`

_private_

```
fn parse_rejects_missing_begin_marker()
```

**Calls:** parse_patch

### parse_rejects_missing_end_marker

*Rust Function* — `src/llm/tools/apply_patch.rs#L553-L556`

_private_

```
fn parse_rejects_missing_end_marker()
```

**Calls:** parse_patch

### parse_update_with_move_to

*Rust Function* — `src/llm/tools/apply_patch.rs#L586-L597`

_private_

```
fn parse_update_with_move_to()
```

**Calls:** parse_patch

### seeded_context

*Rust Function* — `src/llm/tools/apply_patch.rs#L537-L544`

_private_

```
async fn seeded_context(temp_dir: &TempDir, relative_path: &str) -> ToolExecutionContext
```

**Calls:** record, of

### validate_input_rejects_malformed_patch

*Rust Function* — `src/llm/tools/apply_patch.rs#L857-L861`

_private_

```
fn validate_input_rejects_malformed_patch()
```

### capabilities

*Rust Method* — `src/llm/tools/ask_user.rs#L56-L58`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/ask_user.rs#L33-L37`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/ask_user.rs#L77-L130`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** is_empty, with_metadata

### input_schema

*Rust Method* — `src/llm/tools/ask_user.rs#L39-L54`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/ask_user.rs#L29-L31`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/ask_user.rs#L60-L62`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/ask_user.rs#L64-L75`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** is_empty

### test_auto_approve_returns_placeholder

*Rust Function* — `src/llm/tools/ask_user.rs#L163-L176`

_private_

```
async fn test_auto_approve_returns_placeholder()
```

**Calls:** with_auto_approve

### test_validate_empty_question

*Rust Function* — `src/llm/tools/ask_user.rs#L138-L142`

_private_

```
fn test_validate_empty_question()
```

### test_validate_valid_question

*Rust Function* — `src/llm/tools/ask_user.rs#L145-L150`

_private_

```
fn test_validate_valid_question()
```

### test_validate_with_context

*Rust Function* — `src/llm/tools/ask_user.rs#L153-L160`

_private_

```
fn test_validate_with_context()
```

### capabilities

*Rust Method* — `src/llm/tools/bash.rs#L284-L290`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/bash.rs#L247-L249`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/bash.rs#L309-L426`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** is_read_only_command, resolve_shell, code, is_empty, with_metadata

### input_schema

*Rust Method* — `src/llm/tools/bash.rs#L251-L282`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/bash.rs#L243-L245`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/bash.rs#L292-L294`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/bash.rs#L296-L307`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** is_empty

### bash_runs_posix_in_the_requested_working_directory

*Rust Function* — `src/llm/tools/bash.rs#L464-L497`

_private_

```
async fn bash_runs_posix_in_the_requested_working_directory()
```

### is_read_only_command

*Rust Function* — `src/llm/tools/bash.rs#L123-L239`

_private_

```
fn is_read_only_command(command: &str) -> bool
```

**Calls:** find_active_shell_operator, next, len

**Called by:** execute

### read_only_mode_allows_simple_safe_commands

*Rust Function* — `src/llm/tools/bash.rs#L699-L705`

_private_

```
fn read_only_mode_allows_simple_safe_commands()
```

### read_only_mode_rejects_chained_destructive_commands

*Rust Function* — `src/llm/tools/bash.rs#L680-L686`

_private_

```
fn read_only_mode_rejects_chained_destructive_commands()
```

### read_only_mode_rejects_git_config

*Rust Function* — `src/llm/tools/bash.rs#L730-L734`

_private_

```
fn read_only_mode_rejects_git_config()
```

### read_only_mode_rejects_mutating_find_flags

*Rust Function* — `src/llm/tools/bash.rs#L711-L722`

_private_

```
fn read_only_mode_rejects_mutating_find_flags()
```

### read_only_mode_rejects_network_fetch_tools

*Rust Function* — `src/llm/tools/bash.rs#L693-L696`

_private_

```
fn read_only_mode_rejects_network_fetch_tools()
```

### resolve_shell

*Rust Function* — `src/llm/tools/bash.rs#L37-L77`

_private_

```
fn resolve_shell() -> (String, &'static str)
```

**Called by:** execute, windows_resolves_a_posix_shell_not_cmd

### test_bash_accepts_directory_alias

*Rust Function* — `src/llm/tools/bash.rs#L584-L613`

_private_

```
async fn test_bash_accepts_directory_alias()
```

**Calls:** with_auto_approve

### test_bash_invalid_command

*Rust Function* — `src/llm/tools/bash.rs#L538-L549`

_private_

```
async fn test_bash_invalid_command()
```

**Calls:** with_auto_approve

### test_bash_is_background_notes_synchronous_fallback

*Rust Function* — `src/llm/tools/bash.rs#L641-L660`

_private_

```
async fn test_bash_is_background_notes_synchronous_fallback()
```

**Calls:** with_auto_approve

### test_bash_simple_command

*Rust Function* — `src/llm/tools/bash.rs#L500-L518`

_private_

```
async fn test_bash_simple_command()
```

**Calls:** with_auto_approve

### test_bash_timeout

*Rust Function* — `src/llm/tools/bash.rs#L553-L567`

_private_

```
async fn test_bash_timeout()
```

**Calls:** with_auto_approve, with_timeout

### test_bash_timeout_field_overrides_context_default

*Rust Function* — `src/llm/tools/bash.rs#L619-L635`

_private_

```
async fn test_bash_timeout_field_overrides_context_default()
```

**Calls:** with_auto_approve, with_timeout

### test_bash_tool_schema

*Rust Function* — `src/llm/tools/bash.rs#L570-L578`

_private_

```
fn test_bash_tool_schema()
```

### test_bash_with_exit_code

*Rust Function* — `src/llm/tools/bash.rs#L521-L535`

_private_

```
async fn test_bash_with_exit_code()
```

**Calls:** with_auto_approve

### test_validate_empty_command

*Rust Function* — `src/llm/tools/bash.rs#L663-L671`

_private_

```
fn test_validate_empty_command()
```

### windows_resolves_a_posix_shell_not_cmd

*Rust Function* — `src/llm/tools/bash.rs#L449-L461`

_private_

```
fn windows_resolves_a_posix_shell_not_cmd()
```

**Calls:** resolve_shell

### from_tool

*Rust Method* — `src/llm/tools/cache.rs#L17-L25`

```
pub fn from_tool(tool_name: &str, inputs: &Value) -> Self
```

**Calls:** finish

**Called by:** send_message_with_tools_inner, cache_hit_returns_same_result, cache_expires_after_ttl, invalidate_matching_drops_selected_tools_and_keeps_others, zero_ttl_insert_is_noop

### evict_expired

*Rust Method* — `src/llm/tools/cache.rs#L120-L123`

```
pub fn evict_expired(&self)
```

### get

*Rust Method* — `src/llm/tools/cache.rs#L89-L97`

```
pub fn get(&self, key: &CacheKey) -> Option<String>
```

### insert

*Rust Method* — `src/llm/tools/cache.rs#L100-L111`

```
pub fn insert(&self, key: CacheKey, result: String, ttl: Duration)
```

### insert_for_tool

*Rust Method* — `src/llm/tools/cache.rs#L114-L117`

```
pub fn insert_for_tool(&self, key: CacheKey, result: String)
```

**Calls:** ttl_for

**Called by:** send_message_with_tools_inner

### invalidate_matching

*Rust Method* — `src/llm/tools/cache.rs#L132-L134`

```
pub fn invalidate_matching(&self, pred: impl Fn(&str) -> bool)
```

**Called by:** send_message_with_tools_inner, invalidate_matching_drops_selected_tools_and_keeps_others

### new

*Rust Method* — `src/llm/tools/cache.rs#L81-L86`

```
pub fn new(ttl: ToolTtlConfig) -> Self
```

### default

*Rust Method* — `src/llm/tools/cache.rs#L47-L57`

_private_

```
fn default() -> Self
```

### ttl_for

*Rust Method* — `src/llm/tools/cache.rs#L61-L71`

```
pub fn ttl_for(&self, tool_name: &str) -> Duration
```

**Called by:** insert_for_tool

### cache_expires_after_ttl

*Rust Function* — `src/llm/tools/cache.rs#L154-L164`

_private_

```
async fn cache_expires_after_ttl()
```

**Calls:** from_tool

### cache_hit_returns_same_result

*Rust Function* — `src/llm/tools/cache.rs#L142-L151`

_private_

```
fn cache_hit_returns_same_result()
```

**Calls:** from_tool

### invalidate_matching_drops_selected_tools_and_keeps_others

*Rust Function* — `src/llm/tools/cache.rs#L175-L189`

_private_

```
fn invalidate_matching_drops_selected_tools_and_keeps_others()
```

**Calls:** from_tool, invalidate_matching

### write_tool_not_cached

*Rust Function* — `src/llm/tools/cache.rs#L167-L172`

_private_

```
fn write_tool_not_cached()
```

### zero_ttl_insert_is_noop

*Rust Function* — `src/llm/tools/cache.rs#L192-L197`

_private_

```
fn zero_ttl_insert_is_noop()
```

**Calls:** from_tool

### capabilities

*Rust Method* — `src/llm/tools/code_exec.rs#L81-L87`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/code_exec.rs#L44-L46`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/code_exec.rs#L128-L265`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** code, is_empty

### input_schema

*Rust Method* — `src/llm/tools/code_exec.rs#L48-L79`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/code_exec.rs#L40-L42`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/code_exec.rs#L89-L91`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/code_exec.rs#L93-L126`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** is_empty

### default_timeout

*Rust Function* — `src/llm/tools/code_exec.rs#L34-L36`

_private_

```
fn default_timeout() -> u64
```

### load

*Rust Method* — `src/llm/tools/context.rs#L55-L66`

_private_

```
async fn load(path: &Path, session_id: &str) -> Result<Self>
```

**Calls:** from_str

### new

*Rust Method* — `src/llm/tools/context.rs#L43-L53`

_private_

```
fn new(session_id: String) -> Self
```

### save

*Rust Method* — `src/llm/tools/context.rs#L68-L79`

_private_

```
async fn save(&self, path: &Path) -> Result<()>
```

### capabilities

*Rust Method* — `src/llm/tools/context.rs#L196-L198`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/context.rs#L142-L144`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/context.rs#L210-L413`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** is_empty, len

### input_schema

*Rust Method* — `src/llm/tools/context.rs#L146-L194`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/context.rs#L138-L140`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/context.rs#L200-L202`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/context.rs#L204-L208`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

### get_store_path

*Rust Function* — `src/llm/tools/context.rs#L129-L134`

_private_

```
fn get_store_path(context: &ToolExecutionContext) -> PathBuf
```

### extract_html_title

*Rust Method* — `src/llm/tools/doc_parser.rs#L488-L497`

_private_

```
fn extract_html_title(html: &str) -> Option<String>
```

**Called by:** parse_html, test_extract_html_title

### extract_metadata_from_core_xml

*Rust Method* — `src/llm/tools/doc_parser.rs#L375-L408`

_private_

```
fn extract_metadata_from_core_xml(xml: &str) -> (Option<String>, Option<String>)
```

**Calls:** from_str

**Called by:** parse_docx

### extract_text_from_docx_xml

*Rust Method* — `src/llm/tools/doc_parser.rs#L334-L372`

_private_

```
fn extract_text_from_docx_xml(xml: &str) -> String
```

**Calls:** from_str, is_empty

**Called by:** parse_docx

### parse_docx

*Rust Method* — `src/llm/tools/doc_parser.rs#L286-L331`

_private_

```
async fn parse_docx(&self, path: &Path) -> Result<(String, ParsedMetadata)>
```

**Calls:** extract_text_from_docx_xml, extract_metadata_from_core_xml

**Called by:** execute

### parse_html

*Rust Method* — `src/llm/tools/doc_parser.rs#L426-L441`

_private_

```
async fn parse_html(&self, path: &Path) -> Result<(String, ParsedMetadata)>
```

**Calls:** strip_html_tags, extract_html_title

**Called by:** execute

### parse_json

*Rust Method* — `src/llm/tools/doc_parser.rs#L500-L518`

_private_

```
async fn parse_json(&self, path: &Path) -> Result<(String, ParsedMetadata)>
```

**Called by:** execute

### parse_pdf

*Rust Method* — `src/llm/tools/doc_parser.rs#L231-L283`

_private_

```
async fn parse_pdf( &self, path: &Path, input: &DocParserInput, ) -> Result<(String, ParsedMetadata)>
```

**Calls:** len, is_empty

**Called by:** execute

### parse_text

*Rust Method* — `src/llm/tools/doc_parser.rs#L411-L423`

_private_

```
async fn parse_text(&self, path: &Path, _format: &str) -> Result<(String, ParsedMetadata)>
```

**Called by:** execute

### parse_xml

*Rust Method* — `src/llm/tools/doc_parser.rs#L521-L560`

_private_

```
async fn parse_xml(&self, path: &Path) -> Result<(String, ParsedMetadata)>
```

**Calls:** from_str, is_empty

**Called by:** execute

### strip_html_tags

*Rust Method* — `src/llm/tools/doc_parser.rs#L444-L485`

_private_

```
fn strip_html_tags(html: &str) -> String
```

**Calls:** len, is_empty

**Called by:** parse_html, test_strip_html_tags

### capabilities

*Rust Method* — `src/llm/tools/doc_parser.rs#L88-L90`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/doc_parser.rs#L56-L59`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/doc_parser.rs#L102-L219`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** check_path, len, parse_pdf, parse_docx, parse_text, parse_html, parse_json, parse_xml, with_metadata

### input_schema

*Rust Method* — `src/llm/tools/doc_parser.rs#L61-L86`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/doc_parser.rs#L52-L54`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/doc_parser.rs#L92-L94`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/doc_parser.rs#L96-L100`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

### context_with_file

*Rust Function* — `src/llm/tools/doc_parser.rs#L574-L584`

_private_

```
fn context_with_file(name: &str, content: &str) -> (TempDir, PathBuf, ToolExecutionContext)
```

**Called by:** test_parse_text_file, test_parse_markdown_file, test_parse_json_file, test_parse_html_file, test_max_chars_truncation, test_max_chars_truncation_does_not_panic_on_multibyte_text, test_unsupported_format

### test_extract_html_title

*Rust Function* — `src/llm/tools/doc_parser.rs#L774-L778`

_private_

```
fn test_extract_html_title()
```

**Calls:** extract_html_title

### test_max_chars_truncation

*Rust Function* — `src/llm/tools/doc_parser.rs#L656-L671`

_private_

```
async fn test_max_chars_truncation()
```

**Calls:** context_with_file

### test_max_chars_truncation_does_not_panic_on_multibyte_text

*Rust Function* — `src/llm/tools/doc_parser.rs#L678-L690`

_private_

```
async fn test_max_chars_truncation_does_not_panic_on_multibyte_text()
```

**Calls:** context_with_file

### test_nonexistent_file

*Rust Function* — `src/llm/tools/doc_parser.rs#L707-L721`

_private_

```
async fn test_nonexistent_file()
```

### test_parse_html_file

*Rust Function* — `src/llm/tools/doc_parser.rs#L637-L653`

_private_

```
async fn test_parse_html_file()
```

**Calls:** context_with_file

### test_parse_json_file

*Rust Function* — `src/llm/tools/doc_parser.rs#L621-L634`

_private_

```
async fn test_parse_json_file()
```

**Calls:** context_with_file

### test_parse_markdown_file

*Rust Function* — `src/llm/tools/doc_parser.rs#L605-L618`

_private_

```
async fn test_parse_markdown_file()
```

**Calls:** context_with_file

### test_parse_text_file

*Rust Function* — `src/llm/tools/doc_parser.rs#L587-L602`

_private_

```
async fn test_parse_text_file()
```

**Calls:** context_with_file

### test_path_outside_working_directory_is_denied

*Rust Function* — `src/llm/tools/doc_parser.rs#L730-L751`

_private_

```
async fn test_path_outside_working_directory_is_denied()
```

### test_strip_html_tags

*Rust Function* — `src/llm/tools/doc_parser.rs#L765-L771`

_private_

```
fn test_strip_html_tags()
```

**Calls:** strip_html_tags

### test_tool_schema

*Rust Function* — `src/llm/tools/doc_parser.rs#L754-L762`

_private_

```
fn test_tool_schema()
```

### test_unsupported_format

*Rust Function* — `src/llm/tools/doc_parser.rs#L693-L704`

_private_

```
async fn test_unsupported_format()
```

**Calls:** context_with_file

### capabilities

*Rust Method* — `src/llm/tools/edit.rs#L194-L200`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/edit.rs#L113-L118`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/edit.rs#L212-L414`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** normalize_input, check_path, validate_file_path, of, len, record

### input_schema

*Rust Method* — `src/llm/tools/edit.rs#L120-L192`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/edit.rs#L109-L111`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/edit.rs#L202-L204`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/edit.rs#L206-L210`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** normalize_input

### context

*Rust Function* — `src/llm/tools/edit.rs#L423-L426`

_private_

```
fn context(temp_dir: &TempDir) -> ToolExecutionContext
```

### default_true

*Rust Function* — `src/llm/tools/edit.rs#L78-L80`

_private_

```
fn default_true() -> bool
```

### normalize_input

*Rust Function* — `src/llm/tools/edit.rs#L93-L105`

_private_

```
fn normalize_input(mut input: Value) -> Value
```

**Called by:** validate_input, execute

### seeded_context

*Rust Function* — `src/llm/tools/edit.rs#L433-L440`

_private_

```
async fn seeded_context(temp_dir: &TempDir, relative_path: &str) -> ToolExecutionContext
```

**Calls:** record, of

### test_consecutive_edits_do_not_require_a_re_read_between_them

*Rust Function* — `src/llm/tools/edit.rs#L715-L751`

_private_

```
async fn test_consecutive_edits_do_not_require_a_re_read_between_them()
```

### test_edit_rejects_a_file_changed_since_it_was_read

*Rust Function* — `src/llm/tools/edit.rs#L650-L677`

_private_

```
async fn test_edit_rejects_a_file_changed_since_it_was_read()
```

### test_edit_rejects_a_file_never_read_this_session

*Rust Function* — `src/llm/tools/edit.rs#L628-L644`

_private_

```
async fn test_edit_rejects_a_file_never_read_this_session()
```

### test_line_operation_without_operation_field_is_rejected

*Rust Function* — `src/llm/tools/edit.rs#L571-L585`

_private_

```
async fn test_line_operation_without_operation_field_is_rejected()
```

### test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field

*Rust Function* — `src/llm/tools/edit.rs#L471-L489`

_private_

```
async fn test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field()
```

### test_read_file_then_edit_file_succeeds

*Rust Function* — `src/llm/tools/edit.rs#L683-L708`

_private_

```
async fn test_read_file_then_edit_file_succeeds()
```

### test_replace_all_true_replaces_every_occurrence

*Rust Function* — `src/llm/tools/edit.rs#L522-L544`

_private_

```
async fn test_replace_all_true_replaces_every_occurrence()
```

### test_replace_lines_still_works

*Rust Function* — `src/llm/tools/edit.rs#L588-L612`

_private_

```
async fn test_replace_lines_still_works()
```

### test_replace_missing_text_errors

*Rust Function* — `src/llm/tools/edit.rs#L547-L565`

_private_

```
async fn test_replace_missing_text_errors()
```

### test_replace_rejects_non_unique_match_by_default

*Rust Function* — `src/llm/tools/edit.rs#L496-L519`

_private_

```
async fn test_replace_rejects_non_unique_match_by_default()
```

### test_replace_with_explicit_operation_still_works

*Rust Function* — `src/llm/tools/edit.rs#L443-L463`

_private_

```
async fn test_replace_with_explicit_operation_still_works()
```

### test_validate_input_accepts_file_path_alias

*Rust Function* — `src/llm/tools/edit.rs#L615-L623`

_private_

```
fn test_validate_input_accepts_file_path_alias()
```

### test_tool_error_display

*Rust Function* — `src/llm/tools/error.rs#L190-L196`

_private_

```
fn test_tool_error_display()
```

### validate_directory_path

*Rust Function* — `src/llm/tools/error.rs#L154-L183`

```
pub fn validate_directory_path( requested_path: &str, working_directory: &std::path::Path, ) -> std::result::Result<std::path::PathBuf, String>
```

**Calls:** validate_path_safety

### validate_file_path

*Rust Function* — `src/llm/tools/error.rs#L120-L149`

```
pub fn validate_file_path( requested_path: &str, working_directory: &std::path::Path, ) -> std::result::Result<std::path::PathBuf, String>
```

**Calls:** validate_path_safety

**Called by:** execute, execute, execute

### validate_path_safety

*Rust Function* — `src/llm/tools/error.rs#L58-L110`

```
pub fn validate_path_safety( requested_path: &str, working_directory: &std::path::Path, ) -> Result<std::path::PathBuf>
```

**Called by:** execute, validate_file_path, validate_directory_path, execute

### of

*Rust Method* — `src/llm/tools/file_read_cache.rs#L35-L40`

```
pub fn of(metadata: &std::fs::Metadata) -> Self
```

**Calls:** len

**Called by:** execute, seeded_context, execute_is_atomic_across_files_on_failure, execute, seeded_context, execute, execute, test_overwrite_existing_file, test_overwrite_rejects_a_file_changed_since_it_was_read

### check

*Rust Method* — `src/llm/tools/file_read_cache.rs#L85-L92`

```
pub fn check(&self, path: &Path, current: FileFingerprint) -> ReadGate
```

### new

*Rust Method* — `src/llm/tools/file_read_cache.rs#L66-L68`

```
pub fn new() -> Self
```

### record

*Rust Method* — `src/llm/tools/file_read_cache.rs#L76-L81`

```
pub fn record(&self, path: &Path, fingerprint: FileFingerprint)
```

**Called by:** execute, seeded_context, execute_is_atomic_across_files_on_failure, execute, seeded_context, matching_fingerprint_after_record_is_ok, mismatched_fingerprint_is_stale, distinct_paths_are_tracked_independently, re_recording_updates_the_fingerprint, execute, execute, test_overwrite_existing_file, test_overwrite_rejects_a_file_changed_since_it_was_read

### distinct_paths_are_tracked_independently

*Rust Function* — `src/llm/tools/file_read_cache.rs#L127-L131`

_private_

```
fn distinct_paths_are_tracked_independently()
```

**Calls:** record, fp

### fp

*Rust Function* — `src/llm/tools/file_read_cache.rs#L99-L104`

_private_

```
fn fp(size: u64) -> FileFingerprint
```

**Called by:** matching_fingerprint_after_record_is_ok, mismatched_fingerprint_is_stale, distinct_paths_are_tracked_independently, re_recording_updates_the_fingerprint

### matching_fingerprint_after_record_is_ok

*Rust Function* — `src/llm/tools/file_read_cache.rs#L113-L117`

_private_

```
fn matching_fingerprint_after_record_is_ok()
```

**Calls:** record, fp

### mismatched_fingerprint_is_stale

*Rust Function* — `src/llm/tools/file_read_cache.rs#L120-L124`

_private_

```
fn mismatched_fingerprint_is_stale()
```

**Calls:** record, fp

### never_read_path_is_rejected

*Rust Function* — `src/llm/tools/file_read_cache.rs#L107-L110`

_private_

```
fn never_read_path_is_rejected()
```

### re_recording_updates_the_fingerprint

*Rust Function* — `src/llm/tools/file_read_cache.rs#L134-L140`

_private_

```
fn re_recording_updates_the_fingerprint()
```

**Calls:** record, fp

### capabilities

*Rust Method* — `src/llm/tools/glob.rs#L70-L72`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/glob.rs#L39-L41`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/glob.rs#L91-L191`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** check_path, is_empty, len

### input_schema

*Rust Method* — `src/llm/tools/glob.rs#L43-L68`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/glob.rs#L35-L37`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/glob.rs#L74-L76`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/glob.rs#L78-L89`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** is_empty

### context

*Rust Function* — `src/llm/tools/glob.rs#L200-L203`

_private_

```
fn context(temp_dir: &TempDir) -> ToolExecutionContext
```

### test_glob_matches_recursive_pattern

*Rust Function* — `src/llm/tools/glob.rs#L206-L229`

_private_

```
async fn test_glob_matches_recursive_pattern()
```

### test_glob_no_matches

*Rust Function* — `src/llm/tools/glob.rs#L268-L276`

_private_

```
async fn test_glob_no_matches()
```

### test_glob_respects_gitignore

*Rust Function* — `src/llm/tools/glob.rs#L236-L265`

_private_

```
async fn test_glob_respects_gitignore()
```

### test_glob_respects_limit

*Rust Function* — `src/llm/tools/glob.rs#L279-L293`

_private_

```
async fn test_glob_respects_limit()
```

### search_file

*Rust Method* — `src/llm/tools/grep.rs#L238-L312`

_private_

```
async fn search_file( &self, path: &Path, regex: &regex::Regex, input: &GrepInput, matches: &mut Vec<String>, total_matches: &mut usize, ) -> Result<()>
```

**Calls:** len, skip

**Called by:** execute

### capabilities

*Rust Method* — `src/llm/tools/grep.rs#L120-L122`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/grep.rs#L66-L68`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/grep.rs#L141-L234`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** check_path, search_file, collect_searchable_files, len, is_empty

### input_schema

*Rust Method* — `src/llm/tools/grep.rs#L70-L118`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/grep.rs#L62-L64`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/grep.rs#L124-L126`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/grep.rs#L128-L139`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** is_empty

### collect_searchable_files

*Rust Function* — `src/llm/tools/grep.rs#L324-L340`

_private_

```
async fn collect_searchable_files(dir: &Path) -> Result<Vec<PathBuf>>
```

**Called by:** execute

### default_true

*Rust Function* — `src/llm/tools/grep.rs#L56-L58`

_private_

```
fn default_true() -> bool
```

### test_grep_accepts_glob_alias_for_file_pattern

*Rust Function* — `src/llm/tools/grep.rs#L352-L374`

_private_

```
async fn test_grep_accepts_glob_alias_for_file_pattern()
```

### test_pattern_is_regex_by_default

*Rust Function* — `src/llm/tools/grep.rs#L385-L408`

_private_

```
async fn test_pattern_is_regex_by_default()
```

### test_regex_false_still_searches_literally

*Rust Function* — `src/llm/tools/grep.rs#L413-L436`

_private_

```
async fn test_regex_false_still_searches_literally()
```

### test_search_respects_gitignore

*Rust Function* — `src/llm/tools/grep.rs#L443-L485`

_private_

```
async fn test_search_respects_gitignore()
```

### capabilities

*Rust Method* — `src/llm/tools/http.rs#L129-L131`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/http.rs#L76-L78`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/http.rs#L171-L327`

_private_

```
async fn execute(&self, input: Value, _context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** parse_method, check_url_not_blocked, guard, checked_redirect_policy, is_empty, parse, is_success, text, from_str, len

### input_schema

*Rust Method* — `src/llm/tools/http.rs#L80-L127`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/http.rs#L72-L74`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/http.rs#L133-L135`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/http.rs#L137-L169`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** parse_method

### default_timeout

*Rust Function* — `src/llm/tools/http.rs#L46-L48`

_private_

```
fn default_timeout() -> u64
```

### default_true

*Rust Function* — `src/llm/tools/http.rs#L50-L52`

_private_

```
fn default_true() -> bool
```

### execute_denies_cloud_metadata_endpoint

*Rust Function* — `src/llm/tools/http.rs#L338-L348`

_private_

```
async fn execute_denies_cloud_metadata_endpoint()
```

**Calls:** with_auto_approve

### execute_denies_loopback_address

*Rust Function* — `src/llm/tools/http.rs#L351-L361`

_private_

```
async fn execute_denies_loopback_address()
```

**Calls:** with_auto_approve

### parse_method

*Rust Function* — `src/llm/tools/http.rs#L54-L68`

_private_

```
fn parse_method(method_str: &str) -> Result<Method>
```

**Called by:** validate_input, execute

### list_directory

*Rust Method* — `src/llm/tools/ls.rs#L138-L213`

_private_

```
async fn list_directory( &self, path: &Path, input: &LsInput, output: &mut String, ) -> Result<()>
```

**Calls:** len

**Called by:** execute

### list_recursive

*Rust Method* — `src/llm/tools/ls.rs#L215-L258`

_private_

```
fn list_recursive<'a>( path: &'a PathBuf, input: &'a LsInput, output: &'a mut String, depth: usize, ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>>
```

**Called by:** execute

### capabilities

*Rust Method* — `src/llm/tools/ls.rs#L72-L74`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/ls.rs#L41-L43`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/ls.rs#L86-L134`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** check_path, list_recursive, list_directory

### input_schema

*Rust Method* — `src/llm/tools/ls.rs#L45-L70`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/ls.rs#L37-L39`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/ls.rs#L76-L78`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/ls.rs#L80-L84`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

### capabilities

*Rust Method* — `src/llm/tools/notebook.rs#L143-L149`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/notebook.rs#L94-L96`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/notebook.rs#L161-L335`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** check_path, from_str, len

### input_schema

*Rust Method* — `src/llm/tools/notebook.rs#L98-L141`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/notebook.rs#L90-L92`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/notebook.rs#L151-L153`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/notebook.rs#L155-L159`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

### default_true

*Rust Function* — `src/llm/tools/notebook.rs#L64-L66`

_private_

```
fn default_true() -> bool
```

### minimal_notebook_json

*Rust Function* — `src/llm/tools/notebook.rs#L344-L346`

_private_

```
fn minimal_notebook_json() -> &'static str
```

**Called by:** test_add_cell_within_working_directory_succeeds, test_path_outside_working_directory_is_denied

### test_add_cell_within_working_directory_succeeds

*Rust Function* — `src/llm/tools/notebook.rs#L349-L368`

_private_

```
async fn test_add_cell_within_working_directory_succeeds()
```

**Calls:** minimal_notebook_json

### test_path_outside_working_directory_is_denied

*Rust Function* — `src/llm/tools/notebook.rs#L377-L401`

_private_

```
async fn test_path_outside_working_directory_is_denied()
```

**Calls:** minimal_notebook_json

### test_tool_schema

*Rust Function* — `src/llm/tools/notebook.rs#L404-L408`

_private_

```
fn test_tool_schema()
```

### capabilities

*Rust Method* — `src/llm/tools/plan_tool.rs#L311-L313`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/plan_tool.rs#L201-L205`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/plan_tool.rs#L325-L994`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** validate_plan_file_path, len, from_str, validate_string, is_empty, add_task, validate_dependencies, get_validation_warnings, next_executable_task, execution_summary, get_task_by_order, dependencies_satisfied, get_task_by_order_mut, add_artifact, complete_execution, can_retry, is_complete, add_reflection, record_tool_call, skip

### input_schema

*Rust Method* — `src/llm/tools/plan_tool.rs#L207-L309`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/plan_tool.rs#L197-L199`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/plan_tool.rs#L315-L317`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/plan_tool.rs#L319-L323`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

### acceptance_criteria_are_surfaced_at_start_and_completion

*Rust Function* — `src/llm/tools/plan_tool.rs#L1034-L1089`

_private_

```
async fn acceptance_criteria_are_surfaced_at_start_and_completion()
```

### completing_without_criteria_warns

*Rust Function* — `src/llm/tools/plan_tool.rs#L1094-L1134`

_private_

```
async fn completing_without_criteria_warns()
```

### default_complexity

*Rust Function* — `src/llm/tools/plan_tool.rs#L117-L119`

_private_

```
fn default_complexity() -> u8
```

### default_task_type

*Rust Function* — `src/llm/tools/plan_tool.rs#L121-L123`

_private_

```
fn default_task_type() -> String
```

### sparse_plan_calls_execute_end_to_end

*Rust Function* — `src/llm/tools/plan_tool.rs#L1139-L1167`

_private_

```
async fn sparse_plan_calls_execute_end_to_end()
```

### title_only_create_and_add_task_are_valid

*Rust Function* — `src/llm/tools/plan_tool.rs#L1012-L1026`

_private_

```
fn title_only_create_and_add_task_are_valid()
```

### validate_plan_file_path

*Rust Function* — `src/llm/tools/plan_tool.rs#L127-L164`

_private_

```
fn validate_plan_file_path(path: &Path, working_dir: &Path) -> Result<()>
```

**Calls:** len

**Called by:** execute, test_validate_path_within_working_directory, test_validate_path_outside_working_directory, test_validate_path_traversal_attack, test_validate_filename_pattern, test_validate_filename_requires_uuid, test_validate_symlink_rejection, test_filename_with_special_characters, test_filename_with_null_byte, test_validate_plan_file_path_canonical

### validate_string

*Rust Function* — `src/llm/tools/plan_tool.rs#L175-L193`

_private_

```
fn validate_string(s: &str, max_len: usize, field_name: &str) -> Result<()>
```

**Calls:** is_empty, len

**Called by:** execute, test_validate_string_empty, test_validate_string_whitespace_only, test_validate_string_exceeds_max_length, test_validate_string_valid, test_validate_title_at_limit, test_validate_title_one_over_limit, test_validate_description_at_limit, test_validate_context_at_limit

### test_default_complexity

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L150-L152`

_private_

```
fn test_default_complexity()
```

### test_filename_with_null_byte

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L195-L207`

_private_

```
fn test_filename_with_null_byte()
```

**Calls:** validate_plan_file_path

### test_filename_with_special_characters

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L183-L192`

_private_

```
fn test_filename_with_special_characters()
```

**Calls:** validate_plan_file_path

### test_input_validation_limits

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L142-L147`

_private_

```
fn test_input_validation_limits()
```

### test_max_plan_file_size_constant

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L136-L139`

_private_

```
fn test_max_plan_file_size_constant()
```

### test_validate_context_at_limit

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L176-L180`

_private_

```
fn test_validate_context_at_limit()
```

**Calls:** validate_string

### test_validate_description_at_limit

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L169-L173`

_private_

```
fn test_validate_description_at_limit()
```

**Calls:** validate_string

### test_validate_filename_pattern

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L55-L68`

_private_

```
fn test_validate_filename_pattern()
```

**Calls:** validate_plan_file_path

### test_validate_filename_requires_uuid

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L71-L81`

_private_

```
fn test_validate_filename_requires_uuid()
```

**Calls:** validate_plan_file_path

### test_validate_path_outside_working_directory

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L24-L38`

_private_

```
fn test_validate_path_outside_working_directory()
```

**Calls:** validate_plan_file_path

### test_validate_path_traversal_attack

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L41-L52`

_private_

```
fn test_validate_path_traversal_attack()
```

**Calls:** validate_plan_file_path

### test_validate_path_within_working_directory

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L12-L21`

_private_

```
fn test_validate_path_within_working_directory()
```

**Calls:** validate_plan_file_path

### test_validate_plan_file_path_canonical

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L210-L222`

_private_

```
fn test_validate_plan_file_path_canonical()
```

**Calls:** validate_plan_file_path

### test_validate_string_empty

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L105-L109`

_private_

```
fn test_validate_string_empty()
```

**Calls:** validate_string

### test_validate_string_exceeds_max_length

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L119-L127`

_private_

```
fn test_validate_string_exceeds_max_length()
```

**Calls:** validate_string

### test_validate_string_valid

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L130-L133`

_private_

```
fn test_validate_string_valid()
```

**Calls:** validate_string

### test_validate_string_whitespace_only

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L112-L116`

_private_

```
fn test_validate_string_whitespace_only()
```

**Calls:** validate_string

### test_validate_symlink_rejection

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L85-L102`

_private_

```
fn test_validate_symlink_rejection()
```

**Calls:** validate_plan_file_path

### test_validate_title_at_limit

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L155-L159`

_private_

```
fn test_validate_title_at_limit()
```

**Calls:** validate_string

### test_validate_title_one_over_limit

*Rust Function* — `src/llm/tools/plan_tool_security_tests.rs#L162-L166`

_private_

```
fn test_validate_title_one_over_limit()
```

**Calls:** validate_string

### capabilities

*Rust Method* — `src/llm/tools/powershell.rs#L208-L214`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/powershell.rs#L170-L174`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/powershell.rs#L239-L364`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** is_read_only_powershell, drop, with_metadata, code, is_empty

### input_schema

*Rust Method* — `src/llm/tools/powershell.rs#L176-L206`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/powershell.rs#L166-L168`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/powershell.rs#L216-L218`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/powershell.rs#L220-L237`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** is_empty

### execute_allows_read_only_command_in_plan_mode

*Rust Function* — `src/llm/tools/powershell.rs#L498-L511`

_private_

```
async fn execute_allows_read_only_command_in_plan_mode()
```

**Calls:** make_ctx, with_read_only_mode

### execute_blocks_dangerous_command_in_read_only_mode

*Rust Function* — `src/llm/tools/powershell.rs#L482-L495`

_private_

```
async fn execute_blocks_dangerous_command_in_read_only_mode()
```

**Calls:** make_ctx, with_read_only_mode

### is_read_only_powershell

*Rust Function* — `src/llm/tools/powershell.rs#L128-L134`

_private_

```
fn is_read_only_powershell(command: &str) -> bool
```

**Called by:** execute

### make_ctx

*Rust Function* — `src/llm/tools/powershell.rs#L374-L376`

_private_

```
fn make_ctx() -> ToolExecutionContext
```

**Calls:** with_auto_approve

**Called by:** execute_blocks_dangerous_command_in_read_only_mode, execute_allows_read_only_command_in_plan_mode

### probe_executable

*Rust Function* — `src/llm/tools/powershell.rs#L31-L46`

_private_

```
fn probe_executable(cmd: &str) -> bool
```

### read_only_allows_get_childitem

*Rust Function* — `src/llm/tools/powershell.rs#L386-L388`

_private_

```
fn read_only_allows_get_childitem()
```

### read_only_allows_get_content

*Rust Function* — `src/llm/tools/powershell.rs#L381-L383`

_private_

```
fn read_only_allows_get_content()
```

### read_only_allows_gt_in_string_argument

*Rust Function* — `src/llm/tools/powershell.rs#L426-L431`

_private_

```
fn read_only_allows_gt_in_string_argument()
```

### read_only_allows_select_string

*Rust Function* — `src/llm/tools/powershell.rs#L391-L395`

_private_

```
fn read_only_allows_select_string()
```

### read_only_blocks_append_no_spaces

*Rust Function* — `src/llm/tools/powershell.rs#L440-L443`

_private_

```
fn read_only_blocks_append_no_spaces()
```

### read_only_blocks_iex_without_space

*Rust Function* — `src/llm/tools/powershell.rs#L420-L423`

_private_

```
fn read_only_blocks_iex_without_space()
```

### read_only_blocks_invoke_expression

*Rust Function* — `src/llm/tools/powershell.rs#L403-L405`

_private_

```
fn read_only_blocks_invoke_expression()
```

### read_only_blocks_net_method_call

*Rust Function* — `src/llm/tools/powershell.rs#L413-L417`

_private_

```
fn read_only_blocks_net_method_call()
```

### read_only_blocks_pipe_to_out_file

*Rust Function* — `src/llm/tools/powershell.rs#L408-L410`

_private_

```
fn read_only_blocks_pipe_to_out_file()
```

### read_only_blocks_redirection_with_space

*Rust Function* — `src/llm/tools/powershell.rs#L434-L437`

_private_

```
fn read_only_blocks_redirection_with_space()
```

### read_only_blocks_remove_item

*Rust Function* — `src/llm/tools/powershell.rs#L398-L400`

_private_

```
fn read_only_blocks_remove_item()
```

### tool_metadata

*Rust Function* — `src/llm/tools/powershell.rs#L516-L523`

_private_

```
fn tool_metadata()
```

### validate_accepts_valid_input

*Rust Function* — `src/llm/tools/powershell.rs#L472-L477`

_private_

```
fn validate_accepts_valid_input()
```

### validate_rejects_empty_command

*Rust Function* — `src/llm/tools/powershell.rs#L448-L453`

_private_

```
fn validate_rejects_empty_command()
```

### validate_rejects_timeout_over_600

*Rust Function* — `src/llm/tools/powershell.rs#L464-L469`

_private_

```
fn validate_rejects_timeout_over_600()
```

### validate_rejects_zero_timeout

*Rust Function* — `src/llm/tools/powershell.rs#L456-L461`

_private_

```
fn validate_rejects_zero_timeout()
```

### read_with_buffer

*Rust Method* — `src/llm/tools/read.rs#L161-L241`

_private_

```
async fn read_with_buffer( &self, path: &std::path::Path, start_line: Option<usize>, line_count: Option<usize>, is_large_file: bool, ) -> Result<(String, usize, Option<String>)>
```

**Calls:** is_empty

**Called by:** execute

### capabilities

*Rust Method* — `src/llm/tools/read.rs#L78-L80`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/read.rs#L47-L49`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/read.rs#L92-L156`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** check_path, validate_file_path, len, read_with_buffer, record, of, with_metadata

### input_schema

*Rust Method* — `src/llm/tools/read.rs#L51-L76`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/read.rs#L43-L45`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/read.rs#L82-L84`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/read.rs#L86-L90`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

### test_five_concurrent_reads_no_deadlock

*Rust Function* — `src/llm/tools/read.rs#L355-L401`

_private_

```
async fn test_five_concurrent_reads_no_deadlock()
```

### test_read_file

*Rust Function* — `src/llm/tools/read.rs#L252-L272`

_private_

```
async fn test_read_file()
```

### test_read_file_accepts_file_path_alias

*Rust Function* — `src/llm/tools/read.rs#L323-L342`

_private_

```
async fn test_read_file_accepts_file_path_alias()
```

### test_read_file_line_range

*Rust Function* — `src/llm/tools/read.rs#L275-L299`

_private_

```
async fn test_read_file_line_range()
```

### test_read_nonexistent_file

*Rust Function* — `src/llm/tools/read.rs#L302-L317`

_private_

```
async fn test_read_nonexistent_file()
```

### test_read_tool_schema

*Rust Function* — `src/llm/tools/read.rs#L345-L352`

_private_

```
fn test_read_tool_schema()
```

### capabilities

*Rust Method* — `src/llm/tools/registry.rs#L334-L336`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/registry.rs#L317-L319`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/registry.rs#L342-L348`

_private_

```
async fn execute( &self, _input: Value, _context: &ToolExecutionContext, ) -> Result<ToolResult>
```

### input_schema

*Rust Method* — `src/llm/tools/registry.rs#L321-L332`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/registry.rs#L313-L315`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/registry.rs#L338-L340`

_private_

```
fn requires_approval(&self) -> bool
```

### canonical_name

*Rust Method* — `src/llm/tools/registry.rs#L93-L101`

_private_

```
fn canonical_name<'a>(&self, name: &'a str) -> &'a str
```

**Called by:** is_trusted, get, has_tool, execute

### count

*Rust Method* — `src/llm/tools/registry.rs#L256-L258`

```
pub fn count(&self) -> usize
```

**Calls:** len

### default

*Rust Method* — `src/llm/tools/registry.rs#L262-L264`

_private_

```
fn default() -> Self
```

### execute

*Rust Method* — `src/llm/tools/registry.rs#L139-L220`

```
pub async fn execute( &self, name: &str, input: Value, context: &ToolExecutionContext, ) -> Result<ToolResult>
```

**Calls:** canonical_name

### get

*Rust Method* — `src/llm/tools/registry.rs#L104-L106`

```
pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>>
```

**Calls:** canonical_name

### get_tool_definitions

*Rust Method* — `src/llm/tools/registry.rs#L119-L128`

```
pub fn get_tool_definitions(&self) -> Vec<crate::llm::provider::Tool>
```

**Called by:** send_message_with_tools_inner

### has_tool

*Rust Method* — `src/llm/tools/registry.rs#L109-L111`

```
pub fn has_tool(&self, name: &str) -> bool
```

**Calls:** canonical_name

### is_trusted

*Rust Method* — `src/llm/tools/registry.rs#L67-L74`

```
pub fn is_trusted(&self, name: &str, input: &serde_json::Value) -> bool
```

**Calls:** canonical_name

**Called by:** send_message_with_tools_inner, is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias

### list_tools

*Rust Method* — `src/llm/tools/registry.rs#L114-L116`

```
pub fn list_tools(&self) -> Vec<String>
```

**Called by:** test_list_tools

### new

*Rust Method* — `src/llm/tools/registry.rs#L42-L47`

```
pub fn new() -> Self
```

### register

*Rust Method* — `src/llm/tools/registry.rs#L77-L81`

```
pub fn register(&mut self, tool: Arc<dyn Tool>)
```

**Called by:** build_tool_registry, test_send_message_with_tool_execution, test_register_tool, test_list_tools, test_execute_tool, test_execute_requires_approval, test_execute_with_auto_approve, get_resolves_a_known_alias_to_the_registered_canonical_tool, an_exact_match_wins_over_an_alias_entry, execute_resolves_an_alias_name_to_the_registered_tool, execute_evaluates_policy_against_the_canonical_name_not_the_alias, is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias, create_error_agent, create_test_agent

### register_mcp_server

*Rust Method* — `src/llm/tools/registry.rs#L226-L253`

```
pub async fn register_mcp_server( &mut self, server_name: &str, command: &str, args: &[&str], ) -> anyhow::Result<usize>
```

**Calls:** discover_tools

**Called by:** connect_configured_mcp_servers, cmd_run, register_mcp_server_with_nonexistent_command_fails_gracefully

### set_policy

*Rust Method* — `src/llm/tools/registry.rs#L50-L52`

```
pub fn set_policy(&mut self, policy: Arc<dyn crate::llm::tools::sandbox::PermissionPolicy>)
```

**Called by:** cmd_chat, cmd_run, execute_evaluates_policy_against_the_canonical_name_not_the_alias, is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias

### an_exact_match_wins_over_an_alias_entry

*Rust Function* — `src/llm/tools/registry.rs#L515-L529`

_private_

```
fn an_exact_match_wins_over_an_alias_entry()
```

**Calls:** register

### execute_evaluates_policy_against_the_canonical_name_not_the_alias

*Rust Function* — `src/llm/tools/registry.rs#L578-L602`

_private_

```
async fn execute_evaluates_policy_against_the_canonical_name_not_the_alias()
```

**Calls:** register, set_policy

### execute_reports_not_found_using_the_original_unresolved_name

*Rust Function* — `src/llm/tools/registry.rs#L554-L570`

_private_

```
async fn execute_reports_not_found_using_the_original_unresolved_name()
```

### execute_resolves_an_alias_name_to_the_registered_tool

*Rust Function* — `src/llm/tools/registry.rs#L532-L551`

_private_

```
async fn execute_resolves_an_alias_name_to_the_registered_tool()
```

**Calls:** register

### get_resolves_a_known_alias_to_the_registered_canonical_tool

*Rust Function* — `src/llm/tools/registry.rs#L494-L504`

_private_

```
fn get_resolves_a_known_alias_to_the_registered_canonical_tool()
```

**Calls:** register

### has_tool_is_false_for_an_alias_whose_target_is_not_registered

*Rust Function* — `src/llm/tools/registry.rs#L507-L512`

_private_

```
fn has_tool_is_false_for_an_alias_whose_target_is_not_registered()
```

### is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias

*Rust Function* — `src/llm/tools/registry.rs#L608-L625`

_private_

```
fn is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias()
```

**Calls:** register, set_policy, is_trusted

### preview_input

*Rust Function* — `src/llm/tools/registry.rs#L25-L32`

_private_

```
fn preview_input(input: &Value) -> String
```

**Called by:** preview_input_truncates_a_large_payload, preview_input_truncates_on_char_boundaries

### preview_input_shows_the_command

*Rust Function* — `src/llm/tools/registry.rs#L278-L281`

_private_

```
fn preview_input_shows_the_command()
```

### preview_input_truncates_a_large_payload

*Rust Function* — `src/llm/tools/registry.rs#L284-L294`

_private_

```
fn preview_input_truncates_a_large_payload()
```

**Calls:** preview_input

### preview_input_truncates_on_char_boundaries

*Rust Function* — `src/llm/tools/registry.rs#L299-L303`

_private_

```
fn preview_input_truncates_on_char_boundaries()
```

**Calls:** preview_input

### register_mcp_server_with_nonexistent_command_fails_gracefully

*Rust Function* — `src/llm/tools/registry.rs#L468-L489`

_private_

```
async fn register_mcp_server_with_nonexistent_command_fails_gracefully()
```

**Calls:** register_mcp_server

### test_execute_nonexistent_tool

*Rust Function* — `src/llm/tools/registry.rs#L413-L422`

_private_

```
async fn test_execute_nonexistent_tool()
```

### test_execute_requires_approval

*Rust Function* — `src/llm/tools/registry.rs#L425-L444`

_private_

```
async fn test_execute_requires_approval()
```

**Calls:** register

### test_execute_tool

*Rust Function* — `src/llm/tools/registry.rs#L391-L410`

_private_

```
async fn test_execute_tool()
```

**Calls:** register

### test_execute_with_auto_approve

*Rust Function* — `src/llm/tools/registry.rs#L447-L465`

_private_

```
async fn test_execute_with_auto_approve()
```

**Calls:** register, with_auto_approve

### test_list_tools

*Rust Function* — `src/llm/tools/registry.rs#L372-L388`

_private_

```
fn test_list_tools()
```

**Calls:** register, list_tools

### test_register_tool

*Rust Function* — `src/llm/tools/registry.rs#L358-L369`

_private_

```
fn test_register_tool()
```

**Calls:** register

### test_registry_creation

*Rust Function* — `src/llm/tools/registry.rs#L352-L355`

_private_

```
fn test_registry_creation()
```

### evaluate

*Rust Method* — `src/llm/tools/sandbox.rs#L395-L397`

_private_

```
fn evaluate(&self, _: &str, _: &Value) -> PolicyDecision
```

### new

*Rust Method* — `src/llm/tools/sandbox.rs#L73-L77`

```
pub fn new(pattern: &str) -> Self
```

### evaluate

*Rust Method* — `src/llm/tools/sandbox.rs#L81-L87`

_private_

```
fn evaluate(&self, tool_name: &str, _inputs: &Value) -> PolicyDecision
```

### evaluate

*Rust Method* — `src/llm/tools/sandbox.rs#L336-L354`

_private_

```
fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision
```

### evaluate

*Rust Method* — `src/llm/tools/sandbox.rs#L249-L286`

_private_

```
fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision
```

**Calls:** find_active_shell_operator, next

### new

*Rust Method* — `src/llm/tools/sandbox.rs#L96-L100`

```
pub fn new(raw: &str) -> Self
```

### evaluate

*Rust Method* — `src/llm/tools/sandbox.rs#L104-L119`

_private_

```
fn evaluate(&self, _tool_name: &str, inputs: &Value) -> PolicyDecision
```

**Calls:** normalize_path

### new

*Rust Method* — `src/llm/tools/sandbox.rs#L50-L54`

```
pub fn new(pattern: &str) -> Self
```

### evaluate

*Rust Method* — `src/llm/tools/sandbox.rs#L58-L64`

_private_

```
fn evaluate(&self, tool_name: &str, _inputs: &Value) -> PolicyDecision
```

### evaluate

*Rust Method* — `src/llm/tools/sandbox.rs#L379-L388`

_private_

```
fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision
```

### evaluate

*Rust Method* — `src/llm/tools/sandbox.rs#L362-L372`

_private_

```
fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision
```

**Calls:** is_permitted

### evaluate

*Rust Method* — `src/llm/tools/sandbox.rs#L617-L620`

_private_

```
fn evaluate(&self, _: &str, _: &Value) -> PolicyDecision
```

### evaluate

*Rust Method* — `src/llm/tools/sandbox.rs#L643-L646`

_private_

```
fn evaluate(&self, _: &str, _: &Value) -> PolicyDecision
```

### check

*Rust Method* — `src/llm/tools/sandbox.rs#L129-L186`

_private_

```
fn check(&self, raw: &str) -> PolicyDecision
```

**Calls:** normalize_path, strip_verbatim_prefix, resolve_existing_prefix

### evaluate

*Rust Method* — `src/llm/tools/sandbox.rs#L227-L240`

_private_

```
fn evaluate(&self, _tool_name: &str, inputs: &Value) -> PolicyDecision
```

### is_permitted

*Rust Method* — `src/llm/tools/sandbox.rs#L30-L32`

```
pub fn is_permitted(&self) -> bool
```

**Called by:** evaluate

### absolute_path_outside_root_denied

*Rust Function* — `src/llm/tools/sandbox.rs#L494-L499`

_private_

```
fn absolute_path_outside_root_denied()
```

**Calls:** make_root

### absolute_path_to_nonexistent_file_in_subdir_allowed

*Rust Function* — `src/llm/tools/sandbox.rs#L535-L544`

_private_

```
fn absolute_path_to_nonexistent_file_in_subdir_allowed()
```

### absolute_path_to_nonexistent_file_inside_root_allowed

*Rust Function* — `src/llm/tools/sandbox.rs#L519-L531`

_private_

```
fn absolute_path_to_nonexistent_file_inside_root_allowed()
```

### absolute_path_to_nonexistent_file_outside_root_still_denied

*Rust Function* — `src/llm/tools/sandbox.rs#L549-L562`

_private_

```
fn absolute_path_to_nonexistent_file_outside_root_still_denied()
```

### absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed

*Rust Function* — `src/llm/tools/sandbox.rs#L575-L595`

_private_

```
fn absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed()
```

**Calls:** check_path

### allow_all_never_confers_trust

*Rust Function* — `src/llm/tools/sandbox.rs#L761-L766`

_private_

```
fn allow_all_never_confers_trust()
```

### and_policy_denies_trusted_command_that_a_later_rule_rejects

*Rust Function* — `src/llm/tools/sandbox.rs#L771-L782`

_private_

```
fn and_policy_denies_trusted_command_that_a_later_rule_rejects()
```

### and_policy_does_not_trust_unlisted_program

*Rust Function* — `src/llm/tools/sandbox.rs#L804-L812`

_private_

```
fn and_policy_does_not_trust_unlisted_program()
```

### and_policy_preserves_trust_when_no_rule_denies

*Rust Function* — `src/llm/tools/sandbox.rs#L787-L798`

_private_

```
fn and_policy_preserves_trust_when_no_rule_denies()
```

### and_policy_short_circuits_on_deny

*Rust Function* — `src/llm/tools/sandbox.rs#L611-L634`

_private_

```
fn and_policy_short_circuits_on_deny()
```

### bash_allowlist_allows_unlisted_operator_free_command_to_reach_approval

*Rust Function* — `src/llm/tools/sandbox.rs#L687-L695`

_private_

```
fn bash_allowlist_allows_unlisted_operator_free_command_to_reach_approval()
```

### bash_allowlist_never_trusts_shell_operator_chaining

*Rust Function* — `src/llm/tools/sandbox.rs#L705-L732`

_private_

```
fn bash_allowlist_never_trusts_shell_operator_chaining()
```

### bash_allowlist_permits_quoted_operator_characters

*Rust Function* — `src/llm/tools/sandbox.rs#L735-L755`

_private_

```
fn bash_allowlist_permits_quoted_operator_characters()
```

### bash_allowlist_trusts_listed_prompts_for_unlisted

*Rust Function* — `src/llm/tools/sandbox.rs#L663-L679`

_private_

```
fn bash_allowlist_trusts_listed_prompts_for_unlisted()
```

### check_path

*Rust Function* — `src/llm/tools/sandbox.rs#L406-L425`

```
pub fn check_path(raw: &str, root: &Path) -> Result<(), String>
```

**Called by:** execute, execute, execute, execute, execute, execute, execute, execute, absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed, execute

### deny_path_prefix_allows_unrelated_path

*Rust Function* — `src/llm/tools/sandbox.rs#L858-L867`

_private_

```
fn deny_path_prefix_allows_unrelated_path()
```

**Calls:** make_root

### deny_path_prefix_blocks_matching_path

*Rust Function* — `src/llm/tools/sandbox.rs#L851-L855`

_private_

```
fn deny_path_prefix_blocks_matching_path()
```

### find_active_shell_operator

*Rust Function* — `src/llm/tools/sandbox.rs#L296-L328`

```
pub fn find_active_shell_operator(cmd: &str) -> Option<&'static str>
```

**Calls:** next

**Called by:** is_read_only_command, evaluate

### make_root

*Rust Function* — `src/llm/tools/sandbox.rs#L487-L491`

_private_

```
fn make_root() -> (TempDir, PathBuf)
```

**Called by:** absolute_path_outside_root_denied, valid_path_inside_root_allowed, path_traversal_denied, deny_path_prefix_allows_unrelated_path, symlink_outside_root_denied

### normalize_path

*Rust Function* — `src/llm/tools/sandbox.rs#L430-L442`

_private_

```
fn normalize_path(path: &Path) -> PathBuf
```

**Called by:** evaluate, check, resolve_existing_prefix

### not_policy_inverts_allow

*Rust Function* — `src/llm/tools/sandbox.rs#L826-L832`

_private_

```
fn not_policy_inverts_allow()
```

### not_policy_inverts_trusted_to_deny

*Rust Function* — `src/llm/tools/sandbox.rs#L815-L823`

_private_

```
fn not_policy_inverts_trusted_to_deny()
```

### or_policy_short_circuits_on_allow

*Rust Function* — `src/llm/tools/sandbox.rs#L637-L660`

_private_

```
fn or_policy_short_circuits_on_allow()
```

### path_traversal_denied

*Rust Function* — `src/llm/tools/sandbox.rs#L835-L848`

_private_

```
fn path_traversal_denied()
```

**Calls:** make_root

### resolve_existing_prefix

*Rust Function* — `src/llm/tools/sandbox.rs#L455-L480`

_private_

```
fn resolve_existing_prefix(path: &Path) -> PathBuf
```

**Calls:** normalize_path

**Called by:** check

### strip_verbatim_prefix

*Rust Function* — `src/llm/tools/sandbox.rs#L196-L224`

_private_

```
fn strip_verbatim_prefix(path: &Path) -> PathBuf
```

**Calls:** next, skip

**Called by:** check

### symlink_outside_root_denied

*Rust Function* — `src/llm/tools/sandbox.rs#L871-L884`

_private_

```
fn symlink_outside_root_denied()
```

**Calls:** make_root

### valid_path_inside_root_allowed

*Rust Function* — `src/llm/tools/sandbox.rs#L598-L608`

_private_

```
fn valid_path_inside_root_allowed()
```

**Calls:** make_root

### capabilities

*Rust Method* — `src/llm/tools/save_memory.rs#L94-L96`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/save_memory.rs#L73-L79`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/save_memory.rs#L111-L153`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** is_empty, memory_path, append_fact, with_metadata

### input_schema

*Rust Method* — `src/llm/tools/save_memory.rs#L81-L92`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/save_memory.rs#L69-L71`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/save_memory.rs#L98-L100`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/save_memory.rs#L102-L109`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** is_empty

### append_fact

*Rust Function* — `src/llm/tools/save_memory.rs#L44-L65`

_private_

```
fn append_fact(existing: &str, fact: &str) -> (String, bool)
```

**Calls:** is_empty, next

**Called by:** execute, append_fact_adds_header_to_a_file_that_lacks_one

### append_fact_adds_header_to_a_file_that_lacks_one

*Rust Function* — `src/llm/tools/save_memory.rs#L292-L299`

_private_

```
fn append_fact_adds_header_to_a_file_that_lacks_one()
```

**Calls:** append_fact

### context

*Rust Function* — `src/llm/tools/save_memory.rs#L162-L165`

_private_

```
fn context(temp_dir: &TempDir) -> ToolExecutionContext
```

### execute_appends_to_existing_memory_file

*Rust Function* — `src/llm/tools/save_memory.rs#L191-L207`

_private_

```
async fn execute_appends_to_existing_memory_file()
```

### execute_blocked_in_read_only_mode

*Rust Function* — `src/llm/tools/save_memory.rs#L273-L281`

_private_

```
async fn execute_blocked_in_read_only_mode()
```

### execute_creates_memory_file_with_header_and_fact

*Rust Function* — `src/llm/tools/save_memory.rs#L168-L188`

_private_

```
async fn execute_creates_memory_file_with_header_and_fact()
```

### execute_does_not_duplicate_an_identical_fact

*Rust Function* — `src/llm/tools/save_memory.rs#L211-L230`

_private_

```
async fn execute_does_not_duplicate_an_identical_fact()
```

### memory_path

*Rust Function* — `src/llm/tools/save_memory.rs#L36-L38`

_private_

```
fn memory_path(working_directory: &Path) -> PathBuf
```

**Called by:** execute

### memory_persists_across_different_sessions_in_the_same_directory

*Rust Function* — `src/llm/tools/save_memory.rs#L237-L270`

_private_

```
async fn memory_persists_across_different_sessions_in_the_same_directory()
```

### validate_input_rejects_empty_fact

*Rust Function* — `src/llm/tools/save_memory.rs#L284-L289`

_private_

```
fn validate_input_rejects_empty_fact()
```

### capabilities

*Rust Method* — `src/llm/tools/skill.rs#L69-L71`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/skill.rs#L46-L50`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/skill.rs#L95-L133`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** is_empty, resolve_skill_path, parse_skill_frontmatter_value, with_metadata

### input_schema

*Rust Method* — `src/llm/tools/skill.rs#L52-L67`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/skill.rs#L42-L44`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/skill.rs#L73-L75`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/skill.rs#L77-L93`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** is_empty

### frontmatter_name_matches

*Rust Function* — `src/llm/tools/skill.rs#L272-L277`

_private_

```
fn frontmatter_name_matches(path: &Path, requested: &str) -> bool
```

**Calls:** parse_skill_frontmatter_value

**Called by:** resolve_skill_path

### list_skills

*Rust Function* — `src/llm/tools/skill.rs#L224-L270`

```
pub(crate) fn list_skills(cwd: &Path) -> Vec<SkillListing>
```

**Calls:** skill_lookup_roots, parse_skill_frontmatter_value

**Called by:** list_skills_discovers_project_local_skills_with_frontmatter, list_skills_falls_back_to_directory_name_without_frontmatter_name, list_skills_discovers_legacy_flat_md_files, list_skills_is_sorted_alphabetically_case_insensitive, list_skills_deduplicates_same_name_across_roots, list_skills_does_not_panic_on_a_directory_with_no_skills_dir, open_skills

### list_skills_deduplicates_same_name_across_roots

*Rust Function* — `src/llm/tools/skill.rs#L440-L450`

_private_

```
fn list_skills_deduplicates_same_name_across_roots()
```

**Calls:** list_skills

### list_skills_discovers_legacy_flat_md_files

*Rust Function* — `src/llm/tools/skill.rs#L402-L418`

_private_

```
fn list_skills_discovers_legacy_flat_md_files()
```

**Calls:** list_skills

### list_skills_discovers_project_local_skills_with_frontmatter

*Rust Function* — `src/llm/tools/skill.rs#L367-L384`

_private_

```
fn list_skills_discovers_project_local_skills_with_frontmatter()
```

**Calls:** list_skills

### list_skills_does_not_panic_on_a_directory_with_no_skills_dir

*Rust Function* — `src/llm/tools/skill.rs#L453-L458`

_private_

```
fn list_skills_does_not_panic_on_a_directory_with_no_skills_dir()
```

**Calls:** list_skills

### list_skills_falls_back_to_directory_name_without_frontmatter_name

*Rust Function* — `src/llm/tools/skill.rs#L387-L399`

_private_

```
fn list_skills_falls_back_to_directory_name_without_frontmatter_name()
```

**Calls:** list_skills

### list_skills_is_sorted_alphabetically_case_insensitive

*Rust Function* — `src/llm/tools/skill.rs#L421-L437`

_private_

```
fn list_skills_is_sorted_alphabetically_case_insensitive()
```

**Calls:** list_skills

### parse_skill_frontmatter_value

*Rust Function* — `src/llm/tools/skill.rs#L279-L302`

_private_

```
fn parse_skill_frontmatter_value(contents: &str, key: &str) -> Option<String>
```

**Calls:** next, is_empty

**Called by:** execute, list_skills, frontmatter_name_matches

### push_if_dir

*Rust Function* — `src/llm/tools/skill.rs#L205-L209`

_private_

```
fn push_if_dir(roots: &mut Vec<PathBuf>, path: PathBuf)
```

**Called by:** skill_lookup_roots

### resolve_skill_path

*Rust Function* — `src/llm/tools/skill.rs#L137-L174`

_private_

```
fn resolve_skill_path(name: &str, cwd: &Path) -> std::result::Result<PathBuf, String>
```

**Calls:** skill_lookup_roots, frontmatter_name_matches

**Called by:** execute

### skill_lookup_roots

*Rust Function* — `src/llm/tools/skill.rs#L177-L203`

_private_

```
fn skill_lookup_roots(cwd: &Path) -> Vec<PathBuf>
```

**Calls:** push_if_dir

**Called by:** resolve_skill_path, list_skills

### test_parse_frontmatter_description

*Rust Function* — `src/llm/tools/skill.rs#L309-L315`

_private_

```
fn test_parse_frontmatter_description()
```

### test_parse_frontmatter_missing_key

*Rust Function* — `src/llm/tools/skill.rs#L324-L327`

_private_

```
fn test_parse_frontmatter_missing_key()
```

### test_parse_frontmatter_no_frontmatter

*Rust Function* — `src/llm/tools/skill.rs#L318-L321`

_private_

```
fn test_parse_frontmatter_no_frontmatter()
```

### test_validate_allows_namespaced_skill

*Rust Function* — `src/llm/tools/skill.rs#L353-L357`

_private_

```
fn test_validate_allows_namespaced_skill()
```

### test_validate_empty_skill_name

*Rust Function* — `src/llm/tools/skill.rs#L330-L334`

_private_

```
fn test_validate_empty_skill_name()
```

### test_validate_rejects_dotdot_traversal

*Rust Function* — `src/llm/tools/skill.rs#L344-L350`

_private_

```
fn test_validate_rejects_dotdot_traversal()
```

### test_validate_rejects_null_byte

*Rust Function* — `src/llm/tools/skill.rs#L360-L364`

_private_

```
fn test_validate_rejects_null_byte()
```

### test_validate_valid_skill_name

*Rust Function* — `src/llm/tools/skill.rs#L337-L341`

_private_

```
fn test_validate_valid_skill_name()
```

### resolve

*Rust Method* — `src/llm/tools/ssrf_guard.rs#L87-L126`

_private_

```
fn resolve(&self, name: Name) -> Resolving
```

**Calls:** is_blocked_ip, is_empty

### allows_public_addresses

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L222-L228`

_private_

```
fn allows_public_addresses()
```

### blocks_carrier_grade_nat_range

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L204-L208`

_private_

```
fn blocks_carrier_grade_nat_range()
```

### blocks_cloud_metadata_link_local

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L192-L194`

_private_

```
fn blocks_cloud_metadata_link_local()
```

### blocks_ipv4_mapped_blocked_address

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L217-L219`

_private_

```
fn blocks_ipv4_mapped_blocked_address()
```

### blocks_ipv6_unique_local_and_link_local

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L211-L214`

_private_

```
fn blocks_ipv6_unique_local_and_link_local()
```

### blocks_loopback

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L186-L189`

_private_

```
fn blocks_loopback()
```

### blocks_rfc1918_private_ranges

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L197-L201`

_private_

```
fn blocks_rfc1918_private_ranges()
```

### check_url_not_blocked

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L132-L147`

```
pub fn check_url_not_blocked(url: &str) -> Result<(), String>
```

**Calls:** parse, is_blocked_ip

**Called by:** execute, checked_redirect_policy, execute

### check_url_not_blocked_allows_normal_domain

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L237-L239`

_private_

```
fn check_url_not_blocked_allows_normal_domain()
```

### check_url_not_blocked_rejects_ip_literal_metadata_url

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L231-L234`

_private_

```
fn check_url_not_blocked_rejects_ip_literal_metadata_url()
```

### checked_redirect_policy

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L165-L179`

```
pub fn checked_redirect_policy(max_redirects: usize) -> reqwest::redirect::Policy
```

**Calls:** len, check_url_not_blocked

**Called by:** execute, checked_redirect_policy_blocks_redirect_to_blocked_address, execute

### checked_redirect_policy_blocks_redirect_to_blocked_address

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L253-L290`

_private_

```
async fn checked_redirect_policy_blocks_redirect_to_blocked_address()
```

**Calls:** checked_redirect_policy

### guard

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L150-L152`

```
pub fn guard(builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder
```

**Called by:** execute, execute

### is_blocked_ip

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L29-L34`

```
pub fn is_blocked_ip(ip: &IpAddr) -> bool
```

**Calls:** is_blocked_ipv4, is_blocked_ipv6

**Called by:** resolve, check_url_not_blocked

### is_blocked_ipv4

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L36-L54`

_private_

```
fn is_blocked_ipv4(ip: &Ipv4Addr) -> bool
```

**Called by:** is_blocked_ip, is_blocked_ipv6

### is_blocked_ipv6

*Rust Function* — `src/llm/tools/ssrf_guard.rs#L56-L77`

_private_

```
fn is_blocked_ipv6(ip: &Ipv6Addr) -> bool
```

**Calls:** is_blocked_ipv4

**Called by:** is_blocked_ip

### acquire

*Rust Method* — `src/llm/tools/task.rs#L75-L142`

_private_

```
async fn acquire(store_path: &Path) -> Result<Self>
```

**Called by:** run_migrations, with_lock, execute

### drop

*Rust Method* — `src/llm/tools/task.rs#L153-L157`

_private_

```
fn drop(&mut self)
```

**Called by:** tilde_in_the_database_path_is_expanded_to_home, execute, execute_next_plan_task, test_database_persistence

### release

*Rust Method* — `src/llm/tools/task.rs#L145-L149`

_private_

```
async fn release(&self) -> Result<()>
```

**Called by:** with_lock, execute

### load

*Rust Method* — `src/llm/tools/task.rs#L167-L175`

_private_

```
async fn load(path: &Path) -> Result<Self>
```

**Calls:** from_str

### new

*Rust Method* — `src/llm/tools/task.rs#L161-L165`

_private_

```
fn new() -> Self
```

### save

*Rust Method* — `src/llm/tools/task.rs#L177-L194`

_private_

```
async fn save(&self, path: &Path) -> Result<()>
```

### with_lock

*Rust Method* — `src/llm/tools/task.rs#L197-L217`

_private_

```
async fn with_lock<F, T>(path: &Path, operation: F) -> Result<T> where F: FnOnce(&mut Self) -> Result<T>,
```

**Calls:** acquire, release

**Called by:** execute

### capabilities

*Rust Method* — `src/llm/tools/task.rs#L374-L376`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/task.rs#L313-L315`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/task.rs#L388-L713`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** acquire, release, parse_status, parse_priority, is_empty, with_lock

### input_schema

*Rust Method* — `src/llm/tools/task.rs#L317-L372`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/task.rs#L309-L311`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/task.rs#L378-L380`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/task.rs#L382-L386`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

### get_store_path

*Rust Function* — `src/llm/tools/task.rs#L300-L305`

_private_

```
fn get_store_path(context: &ToolExecutionContext) -> PathBuf
```

### parse_priority

*Rust Function* — `src/llm/tools/task.rs#L273-L284`

_private_

```
fn parse_priority(priority_str: &str) -> Result<TaskPriority>
```

**Called by:** execute

### parse_status

*Rust Function* — `src/llm/tools/task.rs#L286-L298`

_private_

```
fn parse_status(status_str: &str) -> Result<TaskStatus>
```

**Called by:** execute

### short_id

*Rust Function* — `src/llm/tools/task.rs#L64-L66`

_private_

```
fn short_id(id: &str) -> &str
```

### fmt

*Rust Method* — `src/llm/tools/todo_write.rs#L50-L56`

_private_

```
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

### fmt

*Rust Method* — `src/llm/tools/todo_write.rs#L31-L38`

_private_

```
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

### load

*Rust Method* — `src/llm/tools/todo_write.rs#L75-L82`

_private_

```
async fn load(path: &Path) -> Result<Self>
```

**Calls:** from_str

### save

*Rust Method* — `src/llm/tools/todo_write.rs#L84-L90`

_private_

```
async fn save(&self, path: &Path) -> Result<()>
```

### capabilities

*Rust Method* — `src/llm/tools/todo_write.rs#L206-L208`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/todo_write.rs#L158-L163`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/todo_write.rs#L220-L276`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** render_todos, with_metadata, len

### input_schema

*Rust Method* — `src/llm/tools/todo_write.rs#L165-L204`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/todo_write.rs#L154-L156`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/todo_write.rs#L210-L212`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/todo_write.rs#L214-L218`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

### default_priority

*Rust Function* — `src/llm/tools/todo_write.rs#L114-L116`

_private_

```
fn default_priority() -> TodoPriority
```

### render_todos

*Rust Function* — `src/llm/tools/todo_write.rs#L126-L150`

_private_

```
fn render_todos(todos: &[TodoItem]) -> String
```

**Calls:** is_empty

**Called by:** execute, test_render_todos_completed

### test_render_todos_completed

*Rust Function* — `src/llm/tools/todo_write.rs#L289-L301`

_private_

```
fn test_render_todos_completed()
```

**Calls:** render_todos

### test_render_todos_empty

*Rust Function* — `src/llm/tools/todo_write.rs#L284-L286`

_private_

```
fn test_render_todos_empty()
```

### test_validate_read_action

*Rust Function* — `src/llm/tools/todo_write.rs#L304-L308`

_private_

```
fn test_validate_read_action()
```

### test_validate_write_requires_todos

*Rust Function* — `src/llm/tools/todo_write.rs#L311-L316`

_private_

```
fn test_validate_write_requires_todos()
```

### test_validate_write_with_todos

*Rust Function* — `src/llm/tools/todo_write.rs#L319-L328`

_private_

```
fn test_validate_write_with_todos()
```

### requires_approval

*Rust Method* — `src/llm/tools/trait.rs#L197-L208`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/trait.rs#L214-L217`

_private_

```
fn validate_input(&self, _input: &Value) -> Result<()>
```

### new

*Rust Method* — `src/llm/tools/trait.rs#L68-L79`

```
pub fn new(session_id: Uuid) -> Self
```

### with_auto_approve

*Rust Method* — `src/llm/tools/trait.rs#L88-L91`

```
pub fn with_auto_approve(mut self, auto_approve: bool) -> Self
```

**Called by:** send_message_with_tools_inner, test_auto_approve_returns_placeholder, test_bash_simple_command, test_bash_with_exit_code, test_bash_invalid_command, test_bash_timeout, test_bash_accepts_directory_alias, test_bash_timeout_field_overrides_context_default, test_bash_is_background_notes_synchronous_fallback, execute_denies_cloud_metadata_endpoint, execute_denies_loopback_address, make_ctx, test_execute_with_auto_approve, test_execution_context

### with_file_read_cache

*Rust Method* — `src/llm/tools/trait.rs#L114-L117`

```
pub fn with_file_read_cache(mut self, cache: Arc<FileReadCache>) -> Self
```

**Called by:** send_message_with_tools_inner

### with_read_only_mode

*Rust Method* — `src/llm/tools/trait.rs#L100-L103`

```
pub fn with_read_only_mode(mut self, read_only: bool) -> Self
```

**Called by:** send_message_with_tools_inner, execute_blocks_dangerous_command_in_read_only_mode, execute_allows_read_only_command_in_plan_mode

### with_sub_agent_launcher

*Rust Method* — `src/llm/tools/trait.rs#L106-L109`

```
pub fn with_sub_agent_launcher(mut self, launcher: Arc<dyn SubAgentLauncher>) -> Self
```

**Called by:** send_message_with_tools_inner

### with_timeout

*Rust Method* — `src/llm/tools/trait.rs#L94-L97`

```
pub fn with_timeout(mut self, timeout_secs: u64) -> Self
```

**Called by:** test_bash_timeout, test_bash_timeout_field_overrides_context_default, test_execution_context

### with_working_directory

*Rust Method* — `src/llm/tools/trait.rs#L82-L85`

```
pub fn with_working_directory(mut self, dir: std::path::PathBuf) -> Self
```

### error

*Rust Method* — `src/llm/tools/trait.rs#L148-L155`

```
pub fn error(error: String) -> Self
```

### success

*Rust Method* — `src/llm/tools/trait.rs#L138-L145`

```
pub fn success(output: String) -> Self
```

### with_metadata

*Rust Method* — `src/llm/tools/trait.rs#L158-L161`

```
pub fn with_metadata(mut self, key: String, value: String) -> Self
```

**Called by:** execute, execute, execute, execute, execute, execute, execute, execute, execute, test_tool_result_success, execute, execute

### test_execution_context

*Rust Function* — `src/llm/tools/trait.rs#L225-L234`

_private_

```
fn test_execution_context()
```

**Calls:** with_auto_approve, with_timeout

### test_tool_result_error

*Rust Function* — `src/llm/tools/trait.rs#L248-L253`

_private_

```
fn test_tool_result_error()
```

### test_tool_result_success

*Rust Function* — `src/llm/tools/trait.rs#L237-L245`

_private_

```
fn test_tool_result_success()
```

**Calls:** with_metadata

### capabilities

*Rust Method* — `src/llm/tools/web_fetch.rs#L124-L126`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/web_fetch.rs#L86-L90`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/web_fetch.rs#L157-L242`

_private_

```
async fn execute(&self, input: Value, _context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** check_url_not_blocked, guard, checked_redirect_policy, is_success, len, html_to_text, with_metadata

### input_schema

*Rust Method* — `src/llm/tools/web_fetch.rs#L92-L122`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/web_fetch.rs#L82-L84`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/web_fetch.rs#L128-L130`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/web_fetch.rs#L132-L155`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

### default_max_bytes

*Rust Function* — `src/llm/tools/web_fetch.rs#L47-L49`

_private_

```
fn default_max_bytes() -> usize
```

### default_timeout

*Rust Function* — `src/llm/tools/web_fetch.rs#L44-L46`

_private_

```
fn default_timeout() -> u64
```

### default_true

*Rust Function* — `src/llm/tools/web_fetch.rs#L50-L52`

_private_

```
fn default_true() -> bool
```

### execute_denies_cloud_metadata_endpoint

*Rust Function* — `src/llm/tools/web_fetch.rs#L294-L301`

_private_

```
async fn execute_denies_cloud_metadata_endpoint()
```

### execute_denies_loopback_address

*Rust Function* — `src/llm/tools/web_fetch.rs#L304-L311`

_private_

```
async fn execute_denies_loopback_address()
```

### html_to_text

*Rust Function* — `src/llm/tools/web_fetch.rs#L56-L78`

_private_

```
fn html_to_text(html: &str) -> String
```

**Called by:** execute, test_html_to_text_strips_tags, test_html_to_text_strips_script, test_html_to_text_decodes_entities

### test_html_to_text_decodes_entities

*Rust Function* — `src/llm/tools/web_fetch.rs#L268-L272`

_private_

```
fn test_html_to_text_decodes_entities()
```

**Calls:** html_to_text

### test_html_to_text_strips_script

*Rust Function* — `src/llm/tools/web_fetch.rs#L259-L265`

_private_

```
fn test_html_to_text_strips_script()
```

**Calls:** html_to_text

### test_html_to_text_strips_tags

*Rust Function* — `src/llm/tools/web_fetch.rs#L250-L256`

_private_

```
fn test_html_to_text_strips_tags()
```

**Calls:** html_to_text

### test_validate_input_accepts_https

*Rust Function* — `src/llm/tools/web_fetch.rs#L282-L286`

_private_

```
fn test_validate_input_accepts_https()
```

### test_validate_input_rejects_non_http

*Rust Function* — `src/llm/tools/web_fetch.rs#L275-L279`

_private_

```
fn test_validate_input_rejects_non_http()
```

### capabilities

*Rust Method* — `src/llm/tools/web_search.rs#L104-L106`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/web_search.rs#L80-L82`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/web_search.rs#L129-L225`

_private_

```
async fn execute(&self, input: Value, _context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** is_success, is_empty, len

### input_schema

*Rust Method* — `src/llm/tools/web_search.rs#L84-L102`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/web_search.rs#L76-L78`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/web_search.rs#L108-L110`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/web_search.rs#L112-L127`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

**Calls:** is_empty

### default_max_results

*Rust Function* — `src/llm/tools/web_search.rs#L24-L26`

_private_

```
fn default_max_results() -> usize
```

### capabilities

*Rust Method* — `src/llm/tools/write.rs#L70-L75`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/llm/tools/write.rs#L38-L42`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/llm/tools/write.rs#L87-L222`

_private_

```
async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>
```

**Calls:** check_path, validate_path_safety, of, record, with_metadata, len

### input_schema

*Rust Method* — `src/llm/tools/write.rs#L44-L68`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/llm/tools/write.rs#L34-L36`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/llm/tools/write.rs#L77-L79`

_private_

```
fn requires_approval(&self) -> bool
```

### validate_input

*Rust Method* — `src/llm/tools/write.rs#L81-L85`

_private_

```
fn validate_input(&self, input: &Value) -> Result<()>
```

### test_creating_a_new_file_needs_no_prior_read

*Rust Function* — `src/llm/tools/write.rs#L433-L443`

_private_

```
async fn test_creating_a_new_file_needs_no_prior_read()
```

### test_overwrite_existing_file

*Rust Function* — `src/llm/tools/write.rs#L335-L366`

_private_

```
async fn test_overwrite_existing_file()
```

**Calls:** record, of

### test_overwrite_rejects_a_file_changed_since_it_was_read

*Rust Function* — `src/llm/tools/write.rs#L399-L428`

_private_

```
async fn test_overwrite_rejects_a_file_changed_since_it_was_read()
```

**Calls:** record, of

### test_overwrite_rejects_a_file_never_read_this_session

*Rust Function* — `src/llm/tools/write.rs#L373-L394`

_private_

```
async fn test_overwrite_rejects_a_file_never_read_this_session()
```

### test_write_file

*Rust Function* — `src/llm/tools/write.rs#L232-L252`

_private_

```
async fn test_write_file()
```

### test_write_file_accepts_file_path_alias

*Rust Function* — `src/llm/tools/write.rs#L302-L321`

_private_

```
async fn test_write_file_accepts_file_path_alias()
```

### test_write_file_missing_parent_dir

*Rust Function* — `src/llm/tools/write.rs#L279-L296`

_private_

```
async fn test_write_file_missing_parent_dir()
```

### test_write_file_with_create_dirs

*Rust Function* — `src/llm/tools/write.rs#L255-L276`

_private_

```
async fn test_write_file_with_create_dirs()
```

### test_write_then_overwrite_does_not_require_a_re_read

*Rust Function* — `src/llm/tools/write.rs#L448-L477`

_private_

```
async fn test_write_then_overwrite_does_not_require_a_re_read()
```

### test_write_tool_schema

*Rust Function* — `src/llm/tools/write.rs#L324-L332`

_private_

```
fn test_write_tool_schema()
```

### default

*Rust Method* — `src/logging.rs#L33-L43`

_private_

```
fn default() -> Self
```

### new

*Rust Method* — `src/logging.rs#L48-L50`

```
pub fn new() -> Self
```

### with_console_output

*Rust Method* — `src/logging.rs#L74-L77`

```
pub fn with_console_output(mut self, enabled: bool) -> Self
```

**Called by:** test_log_config_builder

### with_debug_mode

*Rust Method* — `src/logging.rs#L53-L59`

```
pub fn with_debug_mode(mut self, enabled: bool) -> Self
```

**Called by:** setup_from_cli, test_log_config_with_debug

### with_log_dir

*Rust Method* — `src/logging.rs#L62-L65`

```
pub fn with_log_dir(mut self, dir: PathBuf) -> Self
```

### with_log_level

*Rust Method* — `src/logging.rs#L68-L71`

```
pub fn with_log_level(mut self, level: Level) -> Self
```

**Called by:** test_log_config_builder

### with_log_prefix

*Rust Method* — `src/logging.rs#L80-L83`

```
pub fn with_log_prefix(mut self, prefix: String) -> Self
```

**Called by:** test_log_config_builder

### empty

*Rust Method* — `src/logging.rs#L101-L103`

_private_

```
fn empty() -> Self
```

**Called by:** init_minimal_logging, key, test_quit_key, test_submit_key, test_model_info_key, test_provider_switch_key, test_copy_response_key, test_paste_clipboard_key, test_toggle_auto_mode_key, test_newline_key

### with_guard

*Rust Method* — `src/logging.rs#L94-L98`

_private_

```
fn with_guard(guard: WorkerGuard) -> Self
```

**Called by:** init_debug_logging

### cleanup_old_logs

*Rust Function* — `src/logging.rs#L266-L296`

```
pub fn cleanup_old_logs(max_age_days: u64) -> Result<usize, Box<dyn std::error::Error>>
```

**Called by:** cmd_logs, main

### debug_filter_is_scoped_to_crustly

*Rust Function* — `src/logging.rs#L381-L392`

_private_

```
fn debug_filter_is_scoped_to_crustly()
```

**Calls:** parse

### debug_log_files_are_findable_by_the_readers

*Rust Function* — `src/logging.rs#L346-L376`

_private_

```
fn debug_log_files_are_findable_by_the_readers()
```

### get_log_path

*Rust Function* — `src/logging.rs#L242-L263`

```
pub fn get_log_path() -> Option<PathBuf>
```

**Called by:** cmd_logs

### init_debug_logging

*Rust Function* — `src/logging.rs#L128-L203`

_private_

```
fn init_debug_logging(config: LogConfig) -> Result<LoggerGuard, Box<dyn std::error::Error>>
```

**Calls:** parse, is_empty, with_guard

**Called by:** init_logging

### init_logging

*Rust Function* — `src/logging.rs#L117-L125`

```
pub fn init_logging(config: LogConfig) -> Result<LoggerGuard, Box<dyn std::error::Error>>
```

**Calls:** init_debug_logging, init_minimal_logging

**Called by:** setup_from_cli

### init_minimal_logging

*Rust Function* — `src/logging.rs#L206-L233`

_private_

```
fn init_minimal_logging(config: LogConfig) -> Result<LoggerGuard, Box<dyn std::error::Error>>
```

**Calls:** parse, compact, empty

**Called by:** init_logging

### setup_from_cli

*Rust Function* — `src/logging.rs#L236-L239`

```
pub fn setup_from_cli(debug: bool) -> Result<LoggerGuard, Box<dyn std::error::Error>>
```

**Calls:** with_debug_mode, init_logging

**Called by:** main

### test_log_config_builder

*Rust Function* — `src/logging.rs#L319-L328`

_private_

```
fn test_log_config_builder()
```

**Calls:** with_log_level, with_console_output, with_log_prefix

### test_log_config_default

*Rust Function* — `src/logging.rs#L303-L309`

_private_

```
fn test_log_config_default()
```

### test_log_config_with_debug

*Rust Function* — `src/logging.rs#L312-L316`

_private_

```
fn test_log_config_with_debug()
```

**Calls:** with_debug_mode

### test_log_dir_in_crustly_folder

*Rust Function* — `src/logging.rs#L331-L336`

_private_

```
fn test_log_dir_in_crustly_folder()
```

### main

*Rust Function* — `src/main.rs#L6-L27`

_private_

```
async fn main() -> Result<()>
```

**Calls:** parse, setup_from_cli, cleanup_old_logs

### call_tool

*Rust Method* — `src/mcp/client.rs#L139-L175`

```
pub async fn call_tool(&mut self, name: &str, arguments: Value) -> Result<String>
```

**Calls:** send_request

**Called by:** execute, unhealthy_client_returns_graceful_error

### connect

*Rust Method* — `src/mcp/client.rs#L85-L126`

```
pub async fn connect(server_name: &str, command: &str, args: &[&str]) -> Result<Self>
```

**Calls:** with_context, send_request

### discover_tools

*Rust Method* — `src/mcp/client.rs#L129-L136`

```
pub async fn discover_tools(&mut self) -> Result<Vec<McpToolDef>>
```

**Calls:** send_request

**Called by:** register_mcp_server

### is_healthy

*Rust Method* — `src/mcp/client.rs#L177-L179`

```
pub fn is_healthy(&self) -> bool
```

### read_response_line

*Rust Method* — `src/mcp/client.rs#L254-L281`

_private_

```
async fn read_response_line(&mut self) -> Result<String>
```

**Calls:** len, with_context

**Called by:** send_request

### send_request

*Rust Method* — `src/mcp/client.rs#L187-L250`

_private_

```
async fn send_request(&mut self, method: &str, params: Option<Value>) -> Result<Value>
```

**Calls:** with_context, read_response_line, match_response_line

**Called by:** connect, discover_tools, call_tool, send_request_skips_a_notification_and_matches_the_response_for_its_own_id, send_request_errors_when_the_server_process_is_gone

### server_name

*Rust Method* — `src/mcp/client.rs#L181-L183`

```
pub fn server_name(&self) -> &str
```

### new

*Rust Method* — `src/mcp/client.rs#L311-L317`

```
pub fn new(server_name: &str, def: McpToolDef, client: Arc<Mutex<MCPClient>>) -> Self
```

**Calls:** namespaced_tool_name

### capabilities

*Rust Method* — `src/mcp/client.rs#L337-L339`

_private_

```
fn capabilities(&self) -> Vec<ToolCapability>
```

### description

*Rust Method* — `src/mcp/client.rs#L326-L328`

_private_

```
fn description(&self) -> &str
```

### execute

*Rust Method* — `src/mcp/client.rs#L361-L371`

_private_

```
async fn execute( &self, input: Value, _ctx: &ToolExecutionContext, ) -> crate::llm::tools::Result<ToolResult>
```

**Calls:** call_tool

### input_schema

*Rust Method* — `src/mcp/client.rs#L330-L335`

_private_

```
fn input_schema(&self) -> Value
```

### name

*Rust Method* — `src/mcp/client.rs#L322-L324`

_private_

```
fn name(&self) -> &str
```

### requires_approval

*Rust Method* — `src/mcp/client.rs#L357-L359`

_private_

```
fn requires_approval(&self) -> bool
```

### match_response_line

*Rust Function* — `src/mcp/client.rs#L48-L60`

_private_

```
fn match_response_line(line: &str, expected_id: u64) -> ResponseMatch
```

**Calls:** from_str

**Called by:** send_request

### matches_a_response_with_the_expected_id

*Rust Function* — `src/mcp/client.rs#L383-L389`

_private_

```
fn matches_a_response_with_the_expected_id()
```

### mcp_tool_always_requires_approval_regardless_of_empty_capabilities

*Rust Function* — `src/mcp/client.rs#L523-L563`

_private_

```
async fn mcp_tool_always_requires_approval_regardless_of_empty_capabilities()
```

### missing_result_defaults_to_null

*Rust Function* — `src/mcp/client.rs#L421-L427`

_private_

```
fn missing_result_defaults_to_null()
```

### namespaced_tool_name

*Rust Function* — `src/mcp/client.rs#L306-L308`

```
pub fn namespaced_tool_name(server_name: &str, tool_name: &str) -> String
```

**Called by:** new, namespaced_tool_name_contains_no_colons, namespaced_tool_name_matches_provider_function_name_pattern

### namespaced_tool_name_contains_no_colons

*Rust Function* — `src/mcp/client.rs#L574-L580`

_private_

```
fn namespaced_tool_name_contains_no_colons()
```

**Calls:** namespaced_tool_name

### namespaced_tool_name_matches_provider_function_name_pattern

*Rust Function* — `src/mcp/client.rs#L593-L601`

_private_

```
fn namespaced_tool_name_matches_provider_function_name_pattern()
```

**Calls:** namespaced_tool_name

### namespaced_tool_name_uses_double_underscore_convention

*Rust Function* — `src/mcp/client.rs#L585-L590`

_private_

```
fn namespaced_tool_name_uses_double_underscore_convention()
```

### send_request_errors_when_the_server_process_is_gone

*Rust Function* — `src/mcp/client.rs#L482-L508`

_private_

```
async fn send_request_errors_when_the_server_process_is_gone()
```

**Calls:** send_request

### send_request_skips_a_notification_and_matches_the_response_for_its_own_id

*Rust Function* — `src/mcp/client.rs#L437-L476`

_private_

```
async fn send_request_skips_a_notification_and_matches_the_response_for_its_own_id()
```

**Calls:** send_request

### skips_a_response_for_a_different_request_id

*Rust Function* — `src/mcp/client.rs#L392-L395`

_private_

```
fn skips_a_response_for_a_different_request_id()
```

### skips_an_id_less_notification

*Rust Function* — `src/mcp/client.rs#L398-L401`

_private_

```
fn skips_an_id_less_notification()
```

### skips_an_unparseable_line

*Rust Function* — `src/mcp/client.rs#L404-L409`

_private_

```
fn skips_an_unparseable_line()
```

### surfaces_a_server_error_for_the_matching_id

*Rust Function* — `src/mcp/client.rs#L412-L418`

_private_

```
fn surfaces_a_server_error_for_the_matching_id()
```

### add_task

*Rust Method* — `src/plan/mod.rs#L79-L82`

```
pub fn add_task(&mut self, task: PlanTask)
```

**Called by:** create_test_plan, test_plan_update, test_plan_with_complex_task_graph, execute, test_add_task, test_get_task, test_get_task_mut, test_count_by_status, test_progress_percentage, test_is_complete, test_topological_sort_no_dependencies, test_topological_sort_with_dependencies, test_topological_sort_circular_dependency, test_validate_dependencies_success, test_validate_dependencies_invalid_reference, test_validate_dependencies_circular, test_complex_dependency_chain, create_test_plan, plan_task_error_marks_task_failed_and_stops_auto_execution, create_multi_task_plan

### approve

*Rust Method* — `src/plan/mod.rs#L182-L186`

```
pub fn approve(&mut self)
```

### complete

*Rust Method* — `src/plan/mod.rs#L201-L204`

```
pub fn complete(&mut self)
```

### count_by_status

*Rust Method* — `src/plan/mod.rs#L159-L161`

```
pub fn count_by_status(&self, status: TaskStatus) -> usize
```

**Called by:** progress_percentage

### dependencies_satisfied

*Rust Method* — `src/plan/mod.rs#L300-L306`

```
pub fn dependencies_satisfied(&self, task: &PlanTask) -> bool
```

**Called by:** execute, ready_tasks

### execution_summary

*Rust Method* — `src/plan/mod.rs#L309-L337`

```
pub fn execution_summary(&self) -> ExecutionSummary
```

**Calls:** len

**Called by:** execute

### get_task

*Rust Method* — `src/plan/mod.rs#L148-L150`

```
pub fn get_task(&self, task_id: &Uuid) -> Option<&PlanTask>
```

### get_task_by_order

*Rust Method* — `src/plan/mod.rs#L289-L291`

```
pub fn get_task_by_order(&self, order: usize) -> Option<&PlanTask>
```

**Called by:** execute

### get_task_by_order_mut

*Rust Method* — `src/plan/mod.rs#L294-L297`

```
pub fn get_task_by_order_mut(&mut self, order: usize) -> Option<&mut PlanTask>
```

**Called by:** execute

### get_task_mut

*Rust Method* — `src/plan/mod.rs#L153-L156`

```
pub fn get_task_mut(&mut self, task_id: &Uuid) -> Option<&mut PlanTask>
```

**Called by:** test_plan_update_task_status, test_get_task_mut, test_plan_state_transition_workflow, test_task_blocking_and_failure_scenarios

### get_validation_warnings

*Rust Method* — `src/plan/mod.rs#L355-L412`

```
pub fn get_validation_warnings(&self) -> Vec<String>
```

**Calls:** len, is_empty

**Called by:** execute

### is_complete

*Rust Method* — `src/plan/mod.rs#L173-L179`

```
pub fn is_complete(&self) -> bool
```

**Calls:** is_empty

**Called by:** execute

### new

*Rust Method* — `src/plan/mod.rs#L60-L76`

```
pub fn new(session_id: Uuid, title: String, description: String) -> Self
```

### next_executable_task

*Rust Method* — `src/plan/mod.rs#L251-L267`

```
pub fn next_executable_task(&self) -> Option<&PlanTask>
```

**Called by:** execute

### next_executable_task_mut

*Rust Method* — `src/plan/mod.rs#L270-L286`

```
pub fn next_executable_task_mut(&mut self) -> Option<&mut PlanTask>
```

### progress_percentage

*Rust Method* — `src/plan/mod.rs#L164-L170`

```
pub fn progress_percentage(&self) -> f32
```

**Calls:** is_empty, count_by_status, len

**Called by:** get_statistics

### ready_tasks

*Rust Method* — `src/plan/mod.rs#L340-L347`

```
pub fn ready_tasks(&self) -> Vec<&PlanTask>
```

**Calls:** dependencies_satisfied

### reject

*Rust Method* — `src/plan/mod.rs#L189-L192`

```
pub fn reject(&mut self)
```

**Called by:** test_plan_rejection, handle_plan_key

### retriable_tasks

*Rust Method* — `src/plan/mod.rs#L350-L352`

```
pub fn retriable_tasks(&self) -> Vec<&PlanTask>
```

**Calls:** can_retry

### start_execution

*Rust Method* — `src/plan/mod.rs#L195-L198`

```
pub fn start_execution(&mut self)
```

### tasks_in_order

*Rust Method* — `src/plan/mod.rs#L86-L145`

```
pub fn tasks_in_order(&self) -> Option<Vec<&PlanTask>>
```

**Calls:** len, is_empty

**Called by:** validate_dependencies, test_topological_sort_no_dependencies, test_topological_sort_with_dependencies, test_topological_sort_circular_dependency, test_complex_dependency_chain, execute_next_plan_task

### validate_dependencies

*Rust Method* — `src/plan/mod.rs#L208-L247`

```
pub fn validate_dependencies(&self) -> Result<(), String>
```

**Calls:** tasks_in_order, is_empty

**Called by:** execute, test_validate_dependencies_success, test_validate_dependencies_invalid_reference, test_validate_dependencies_circular, test_complex_dependency_chain

### advance

*Rust Method* — `src/plan/mod.rs#L898-L934`

```
pub fn advance(self) -> Self
```

**Called by:** advance_transitions_through_tasks_to_done

### approve

*Rust Method* — `src/plan/mod.rs#L879-L895`

```
pub fn approve(plan_id: uuid::Uuid, tasks: Vec<PlanTask>, auto_plan: bool) -> Self
```

**Calls:** len

### is_high_risk_tool

*Rust Method* — `src/plan/mod.rs#L937-L942`

```
pub fn is_high_risk_tool(tool_name: &str) -> bool
```

**Called by:** auto_mode_bypasses_approval, tool_needs_approval

### tool_needs_approval

*Rust Method* — `src/plan/mod.rs#L864-L874`

```
pub fn tool_needs_approval(&self, tool_name: &str, _threshold: u8) -> bool
```

**Calls:** is_high_risk_tool

### fmt

*Rust Method* — `src/plan/mod.rs#L450-L460`

_private_

```
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

### add_artifact

*Rust Method* — `src/plan/mod.rs#L663-L665`

```
pub fn add_artifact(&mut self, artifact: String)
```

**Called by:** execute

### add_reflection

*Rust Method* — `src/plan/mod.rs#L658-L660`

```
pub fn add_reflection(&mut self, reflection: String)
```

**Called by:** execute

### block

*Rust Method* — `src/plan/mod.rs#L692-L694`

```
pub fn block(&mut self, reason: String)
```

**Called by:** test_task_blocking, render_header, render_chat, render_sessions, render_skills, render_mcp, render_help, render_plan_help, render_plan, render_settings, render_approval, render_file_picker, render_model_info, render_provider_switch, render_model_download, render_model_download_progress, render_model_download_confirm_delete, render_model_download_deleting, render_splash_content

### can_retry

*Rust Method* — `src/plan/mod.rs#L668-L671`

```
pub fn can_retry(&self) -> bool
```

**Called by:** execute, retriable_tasks

### complete

*Rust Method* — `src/plan/mod.rs#L679-L683`

```
pub fn complete(&mut self, notes: Option<String>)
```

### complete_execution

*Rust Method* — `src/plan/mod.rs#L616-L635`

```
pub fn complete_execution(&mut self, output: String, success: bool)
```

**Called by:** execute

### complexity_stars

*Rust Method* — `src/plan/mod.rs#L705-L709`

```
pub fn complexity_stars(&self) -> String
```

### fail

*Rust Method* — `src/plan/mod.rs#L686-L689`

```
pub fn fail(&mut self, reason: String)
```

**Called by:** test_task_failure

### fail_execution

*Rust Method* — `src/plan/mod.rs#L638-L655`

```
pub fn fail_execution(&mut self, error: String)
```

### last_execution

*Rust Method* — `src/plan/mod.rs#L674-L676`

```
pub fn last_execution(&self) -> Option<&TaskExecution>
```

### new

*Rust Method* — `src/plan/mod.rs#L567-L586`

```
pub fn new(order: usize, title: String, description: String, task_type: TaskType) -> Self
```

### record_tool_call

*Rust Method* — `src/plan/mod.rs#L609-L613`

```
pub fn record_tool_call(&mut self, tool_call: ToolCall)
```

**Called by:** execute

### skip

*Rust Method* — `src/plan/mod.rs#L697-L702`

```
pub fn skip(&mut self, reason: Option<String>)
```

**Called by:** parse_native_qwen_tool_calls, search_file, execute, strip_verbatim_prefix, test_task_skip, render_file_picker

### start

*Rust Method* — `src/plan/mod.rs#L589-L591`

```
pub fn start(&mut self)
```

**Called by:** test_task_state_transitions

### start_execution

*Rust Method* — `src/plan/mod.rs#L594-L606`

```
pub fn start_execution(&mut self) -> &mut TaskExecution
```

### icon

*Rust Method* — `src/plan/mod.rs#L786-L795`

```
pub fn icon(&self) -> &str
```

### fmt

*Rust Method* — `src/plan/mod.rs#L772-L781`

_private_

```
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

### fmt

*Rust Method* — `src/plan/mod.rs#L738-L751`

_private_

```
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

### default_max_retries

*Rust Function* — `src/plan/mod.rs#L520-L522`

_private_

```
fn default_max_retries() -> u8
```

### auto_run_no_dialogs

*Rust Function* — `src/plan/plan_tests.rs#L540-L567`

_private_

```
fn auto_run_no_dialogs()
```

### create_test_plan

*Rust Function* — `src/plan/plan_tests.rs#L11-L17`

_private_

```
fn create_test_plan(session_id: Uuid) -> PlanDocument
```

### create_test_task

*Rust Function* — `src/plan/plan_tests.rs#L20-L27`

_private_

```
fn create_test_task(order: usize, title: &str) -> PlanTask
```

**Called by:** test_add_task, test_get_task, test_get_task_mut, test_count_by_status, test_progress_percentage, test_is_complete, test_topological_sort_no_dependencies, test_topological_sort_with_dependencies, test_topological_sort_circular_dependency, test_validate_dependencies_success, test_validate_dependencies_invalid_reference, test_validate_dependencies_circular, test_task_state_transitions, test_task_failure, test_task_blocking, test_task_skip, test_complex_dependency_chain

### test_add_task

*Rust Function* — `src/plan/plan_tests.rs#L45-L58`

_private_

```
fn test_add_task()
```

**Calls:** create_test_task, add_task

### test_complex_dependency_chain

*Rust Function* — `src/plan/plan_tests.rs#L478-L537`

_private_

```
fn test_complex_dependency_chain()
```

**Calls:** create_test_task, add_task, validate_dependencies, tasks_in_order

### test_count_by_status

*Rust Function* — `src/plan/plan_tests.rs#L94-L113`

_private_

```
fn test_count_by_status()
```

**Calls:** create_test_task, add_task

### test_get_task

*Rust Function* — `src/plan/plan_tests.rs#L61-L74`

_private_

```
fn test_get_task()
```

**Calls:** create_test_task, add_task

### test_get_task_mut

*Rust Function* — `src/plan/plan_tests.rs#L77-L91`

_private_

```
fn test_get_task_mut()
```

**Calls:** create_test_task, add_task, get_task_mut

### test_is_complete

*Rust Function* — `src/plan/plan_tests.rs#L145-L177`

_private_

```
fn test_is_complete()
```

**Calls:** create_test_task, add_task

### test_plan_document_new

*Rust Function* — `src/plan/plan_tests.rs#L30-L42`

_private_

```
fn test_plan_document_new()
```

### test_plan_rejection

*Rust Function* — `src/plan/plan_tests.rs#L201-L207`

_private_

```
fn test_plan_rejection()
```

**Calls:** reject

### test_plan_state_transitions

*Rust Function* — `src/plan/plan_tests.rs#L180-L198`

_private_

```
fn test_plan_state_transitions()
```

### test_plan_status_display

*Rust Function* — `src/plan/plan_tests.rs#L464-L475`

_private_

```
fn test_plan_status_display()
```

### test_progress_percentage

*Rust Function* — `src/plan/plan_tests.rs#L116-L142`

_private_

```
fn test_progress_percentage()
```

**Calls:** create_test_task, add_task

### test_task_blocking

*Rust Function* — `src/plan/plan_tests.rs#L359-L370`

_private_

```
fn test_task_blocking()
```

**Calls:** create_test_task, block

### test_task_complexity_stars

*Rust Function* — `src/plan/plan_tests.rs#L382-L421`

_private_

```
fn test_task_complexity_stars()
```

### test_task_failure

*Rust Function* — `src/plan/plan_tests.rs#L349-L356`

_private_

```
fn test_task_failure()
```

**Calls:** create_test_task, fail

### test_task_skip

*Rust Function* — `src/plan/plan_tests.rs#L373-L379`

_private_

```
fn test_task_skip()
```

**Calls:** create_test_task, skip

### test_task_state_transitions

*Rust Function* — `src/plan/plan_tests.rs#L333-L346`

_private_

```
fn test_task_state_transitions()
```

**Calls:** create_test_task, start

### test_task_status_display

*Rust Function* — `src/plan/plan_tests.rs#L441-L451`

_private_

```
fn test_task_status_display()
```

### test_task_status_icons

*Rust Function* — `src/plan/plan_tests.rs#L454-L461`

_private_

```
fn test_task_status_icons()
```

### test_task_type_display

*Rust Function* — `src/plan/plan_tests.rs#L424-L438`

_private_

```
fn test_task_type_display()
```

### test_topological_sort_circular_dependency

*Rust Function* — `src/plan/plan_tests.rs#L257-L277`

_private_

```
fn test_topological_sort_circular_dependency()
```

**Calls:** create_test_task, add_task, tasks_in_order

### test_topological_sort_no_dependencies

*Rust Function* — `src/plan/plan_tests.rs#L210-L226`

_private_

```
fn test_topological_sort_no_dependencies()
```

**Calls:** create_test_task, add_task, tasks_in_order

### test_topological_sort_with_dependencies

*Rust Function* — `src/plan/plan_tests.rs#L229-L254`

_private_

```
fn test_topological_sort_with_dependencies()
```

**Calls:** create_test_task, add_task, tasks_in_order

### test_validate_dependencies_circular

*Rust Function* — `src/plan/plan_tests.rs#L311-L330`

_private_

```
fn test_validate_dependencies_circular()
```

**Calls:** create_test_task, add_task, validate_dependencies

### test_validate_dependencies_invalid_reference

*Rust Function* — `src/plan/plan_tests.rs#L297-L308`

_private_

```
fn test_validate_dependencies_invalid_reference()
```

**Calls:** create_test_task, add_task, validate_dependencies

### test_validate_dependencies_success

*Rust Function* — `src/plan/plan_tests.rs#L280-L294`

_private_

```
fn test_validate_dependencies_success()
```

**Calls:** create_test_task, add_task, validate_dependencies

### new

*Rust Method* — `src/services/mod.rs#L28-L32`

```
pub fn new(pool: Pool) -> Self
```

### pool

*Rust Method* — `src/services/mod.rs#L35-L37`

```
pub fn pool(&self) -> Pool
```

### context

*Rust Method* — `src/services/mod.rs#L84-L86`

```
pub fn context(&self) -> &ServiceContext
```

### files

*Rust Method* — `src/services/mod.rs#L74-L76`

```
pub fn files(&self) -> &FileService
```

**Called by:** test_service_manager_creation

### messages

*Rust Method* — `src/services/mod.rs#L69-L71`

```
pub fn messages(&self) -> &MessageService
```

**Called by:** test_service_manager_creation

### new

*Rust Method* — `src/services/mod.rs#L51-L61`

```
pub fn new(pool: Pool) -> Self
```

### plans

*Rust Method* — `src/services/mod.rs#L79-L81`

```
pub fn plans(&self) -> &PlanService
```

### sessions

*Rust Method* — `src/services/mod.rs#L64-L66`

```
pub fn sessions(&self) -> &SessionService
```

**Called by:** test_service_manager_creation

### create_test_pool

*Rust Function* — `src/services/mod.rs#L94-L100`

_private_

```
async fn create_test_pool() -> Pool
```

**Calls:** run_migrations

### count_files_in_session

*Rust Method* — `src/services/file.rs#L129-L134`

```
pub async fn count_files_in_session(&self, session_id: Uuid) -> Result<i64>
```

**Called by:** test_count_files_in_session

### delete_file

*Rust Method* — `src/services/file.rs#L109-L115`

```
pub async fn delete_file(&self, id: Uuid) -> Result<()>
```

**Called by:** test_delete_file

### delete_files_for_session

*Rust Method* — `src/services/file.rs#L118-L126`

```
pub async fn delete_files_for_session(&self, session_id: Uuid) -> Result<()>
```

**Called by:** test_delete_files_for_session

### find_file_by_path

*Rust Method* — `src/services/file.rs#L70-L75`

```
pub async fn find_file_by_path(&self, session_id: Uuid, path: &Path) -> Result<Option<File>>
```

**Calls:** find_by_path

**Called by:** is_file_tracked, get_or_create_file, test_find_file_by_path

### get_file

*Rust Method* — `src/services/file.rs#L49-L52`

```
pub async fn get_file(&self, id: Uuid) -> Result<Option<File>>
```

**Called by:** get_file_required, test_get_file, test_delete_file

### get_file_required

*Rust Method* — `src/services/file.rs#L55-L59`

```
pub async fn get_file_required(&self, id: Uuid) -> Result<File>
```

**Calls:** get_file

**Called by:** update_file_content, test_update_file_content

### get_files_with_content

*Rust Method* — `src/services/file.rs#L159-L162`

```
pub async fn get_files_with_content(&self, session_id: Uuid) -> Result<Vec<File>>
```

**Calls:** list_files_for_session

**Called by:** test_get_files_with_content

### get_files_without_content

*Rust Method* — `src/services/file.rs#L165-L168`

```
pub async fn get_files_without_content(&self, session_id: Uuid) -> Result<Vec<File>>
```

**Calls:** list_files_for_session

**Called by:** test_get_files_with_content

### get_or_create_file

*Rust Method* — `src/services/file.rs#L143-L156`

```
pub async fn get_or_create_file( &self, session_id: Uuid, path: PathBuf, content: Option<String>, ) -> Result<File>
```

**Calls:** find_file_by_path

**Called by:** test_get_or_create_file

### is_file_tracked

*Rust Method* — `src/services/file.rs#L137-L140`

```
pub async fn is_file_tracked(&self, session_id: Uuid, path: &Path) -> Result<bool>
```

**Calls:** find_file_by_path

**Called by:** test_is_file_tracked

### list_files_for_session

*Rust Method* — `src/services/file.rs#L62-L67`

```
pub async fn list_files_for_session(&self, session_id: Uuid) -> Result<Vec<File>>
```

**Called by:** get_files_with_content, get_files_without_content, test_list_files_for_session, test_delete_files_for_session

### new

*Rust Method* — `src/services/file.rs#L20-L22`

```
pub fn new(context: ServiceContext) -> Self
```

### track_file

*Rust Method* — `src/services/file.rs#L25-L46`

```
pub async fn track_file( &self, session_id: Uuid, path: PathBuf, content: Option<String>, ) -> Result<File>
```

### update_file

*Rust Method* — `src/services/file.rs#L78-L91`

```
pub async fn update_file(&self, file: &File) -> Result<()>
```

### update_file_content

*Rust Method* — `src/services/file.rs#L94-L106`

```
pub async fn update_file_content(&self, id: Uuid, content: Option<String>) -> Result<()>
```

**Calls:** get_file_required

**Called by:** test_update_file_content

### create_test_service

*Rust Function* — `src/services/file.rs#L176-L188`

_private_

```
async fn create_test_service() -> (FileService, SessionService)
```

**Calls:** run_migrations

### test_count_files_in_session

*Rust Function* — `src/services/file.rs#L347-L368`

_private_

```
async fn test_count_files_in_session()
```

**Calls:** count_files_in_session

### test_delete_file

*Rust Function* — `src/services/file.rs#L299-L315`

_private_

```
async fn test_delete_file()
```

**Calls:** delete_file, get_file

### test_delete_files_for_session

*Rust Function* — `src/services/file.rs#L318-L344`

_private_

```
async fn test_delete_files_for_session()
```

**Calls:** delete_files_for_session, list_files_for_session

### test_find_file_by_path

*Rust Function* — `src/services/file.rs#L255-L274`

_private_

```
async fn test_find_file_by_path()
```

**Calls:** find_file_by_path

### test_get_file

*Rust Function* — `src/services/file.rs#L213-L228`

_private_

```
async fn test_get_file()
```

**Calls:** get_file

### test_get_files_with_content

*Rust Function* — `src/services/file.rs#L423-L454`

_private_

```
async fn test_get_files_with_content()
```

**Calls:** get_files_with_content, get_files_without_content

### test_get_or_create_file

*Rust Function* — `src/services/file.rs#L398-L420`

_private_

```
async fn test_get_or_create_file()
```

**Calls:** get_or_create_file

### test_is_file_tracked

*Rust Function* — `src/services/file.rs#L371-L395`

_private_

```
async fn test_is_file_tracked()
```

**Calls:** is_file_tracked

### test_list_files_for_session

*Rust Function* — `src/services/file.rs#L231-L252`

_private_

```
async fn test_list_files_for_session()
```

**Calls:** list_files_for_session

### test_track_file

*Rust Function* — `src/services/file.rs#L191-L210`

_private_

```
async fn test_track_file()
```

### test_update_file_content

*Rust Function* — `src/services/file.rs#L277-L296`

_private_

```
async fn test_update_file_content()
```

**Calls:** update_file_content, get_file_required

### calculate_total_cost

*Rust Method* — `src/services/message.rs#L192-L196`

```
pub async fn calculate_total_cost(&self, session_id: Uuid) -> Result<f64>
```

**Calls:** list_messages_for_session

**Called by:** test_calculate_totals

### calculate_total_tokens

*Rust Method* — `src/services/message.rs#L185-L189`

```
pub async fn calculate_total_tokens(&self, session_id: Uuid) -> Result<i32>
```

**Calls:** list_messages_for_session

**Called by:** test_calculate_totals

### count_messages_in_session

*Rust Method* — `src/services/message.rs#L165-L170`

```
pub async fn count_messages_in_session(&self, session_id: Uuid) -> Result<i64>
```

**Called by:** test_count_messages_in_session

### create_message

*Rust Method* — `src/services/message.rs#L24-L60`

```
pub async fn create_message( &self, session_id: Uuid, role: String, content: String, ) -> Result<Message>
```

**Called by:** send_message, send_message_with_tools_inner, prepare_message_context, create_then_update_survives_a_file_backed_wal_pool, test_create_message, test_get_message, test_list_messages_for_session, test_update_message_usage, test_update_message_metrics_with_perf_data, test_update_message_metrics_without_perf_data, test_delete_message, test_delete_messages_for_session, test_count_messages_in_session, test_get_last_message, test_get_messages_by_role, test_calculate_totals, clear_session_is_refused_while_the_current_session_is_processing, clear_session_proceeds_when_only_another_session_is_processing

### delete_message

*Rust Method* — `src/services/message.rs#L145-L151`

```
pub async fn delete_message(&self, id: Uuid) -> Result<()>
```

**Called by:** test_delete_message

### delete_messages_for_session

*Rust Method* — `src/services/message.rs#L154-L162`

```
pub async fn delete_messages_for_session(&self, session_id: Uuid) -> Result<()>
```

**Called by:** test_delete_messages_for_session, clear_session

### get_last_message

*Rust Method* — `src/services/message.rs#L173-L176`

```
pub async fn get_last_message(&self, session_id: Uuid) -> Result<Option<Message>>
```

**Calls:** list_messages_for_session

### get_message

*Rust Method* — `src/services/message.rs#L63-L66`

```
pub async fn get_message(&self, id: Uuid) -> Result<Option<Message>>
```

**Called by:** get_message_required, test_get_message, test_delete_message

### get_message_required

*Rust Method* — `src/services/message.rs#L69-L73`

```
pub async fn get_message_required(&self, id: Uuid) -> Result<Message>
```

**Calls:** get_message

**Called by:** update_message_usage, update_message_metrics, test_update_message_usage, test_update_message_metrics_with_perf_data, test_update_message_metrics_without_perf_data

### get_messages_by_role

*Rust Method* — `src/services/message.rs#L179-L182`

```
pub async fn get_messages_by_role(&self, session_id: Uuid, role: &str) -> Result<Vec<Message>>
```

**Calls:** list_messages_for_session

**Called by:** test_get_messages_by_role

### list_messages_for_session

*Rust Method* — `src/services/message.rs#L76-L81`

```
pub async fn list_messages_for_session(&self, session_id: Uuid) -> Result<Vec<Message>>
```

**Called by:** send_message_with_tools_inner, prepare_message_context, get_last_message, get_messages_by_role, calculate_total_tokens, calculate_total_cost, test_list_messages_for_session, test_delete_messages_for_session, load_session, clear_session_is_refused_while_the_current_session_is_processing, clear_session_proceeds_when_only_another_session_is_processing, test_end_to_end_simple_message, test_end_to_end_multi_turn_conversation, test_end_to_end_session_management, test_end_to_end_token_usage

### new

*Rust Method* — `src/services/message.rs#L19-L21`

```
pub fn new(context: ServiceContext) -> Self
```

### update_message

*Rust Method* — `src/services/message.rs#L84-L92`

```
pub async fn update_message(&self, message: &Message) -> Result<()>
```

### update_message_metrics

*Rust Method* — `src/services/message.rs#L118-L142`

```
pub async fn update_message_metrics( &self, id: Uuid, provider_name: &str, perf_metrics: Option<&crate::llm::provider::PerfMetrics>, ) -> Result<()>
```

**Calls:** get_message_required

**Called by:** send_message, send_message_with_tools_inner, test_update_message_metrics_with_perf_data, test_update_message_metrics_without_perf_data

### update_message_usage

*Rust Method* — `src/services/message.rs#L95-L112`

```
pub async fn update_message_usage(&self, id: Uuid, token_count: i32, cost: f64) -> Result<()>
```

**Calls:** get_message_required

**Called by:** send_message, send_message_with_tools_inner, create_then_update_survives_a_file_backed_wal_pool, test_update_message_usage, test_calculate_totals

### create_test_service

*Rust Function* — `src/services/message.rs#L204-L216`

_private_

```
async fn create_test_service() -> (MessageService, SessionService)
```

**Calls:** run_migrations

### create_then_update_survives_a_file_backed_wal_pool

*Rust Function* — `src/services/message.rs#L229-L259`

_private_

```
async fn create_then_update_survives_a_file_backed_wal_pool()
```

**Calls:** run_migrations, create_message, update_message_usage

### test_calculate_totals

*Rust Function* — `src/services/message.rs#L544-L580`

_private_

```
async fn test_calculate_totals()
```

**Calls:** create_message, update_message_usage, calculate_total_tokens, calculate_total_cost

### test_count_messages_in_session

*Rust Function* — `src/services/message.rs#L460-L481`

_private_

```
async fn test_count_messages_in_session()
```

**Calls:** create_message, count_messages_in_session

### test_create_message

*Rust Function* — `src/services/message.rs#L262-L278`

_private_

```
async fn test_create_message()
```

**Calls:** create_message

### test_delete_message

*Rust Function* — `src/services/message.rs#L412-L428`

_private_

```
async fn test_delete_message()
```

**Calls:** create_message, delete_message, get_message

### test_delete_messages_for_session

*Rust Function* — `src/services/message.rs#L431-L457`

_private_

```
async fn test_delete_messages_for_session()
```

**Calls:** create_message, delete_messages_for_session, list_messages_for_session

### test_get_last_message

*Rust Function* — `src/services/message.rs#L484-L503`

_private_

```
async fn test_get_last_message()
```

**Calls:** create_message

### test_get_message

*Rust Function* — `src/services/message.rs#L281-L296`

_private_

```
async fn test_get_message()
```

**Calls:** create_message, get_message

### test_get_messages_by_role

*Rust Function* — `src/services/message.rs#L506-L541`

_private_

```
async fn test_get_messages_by_role()
```

**Calls:** create_message, get_messages_by_role

### test_list_messages_for_session

*Rust Function* — `src/services/message.rs#L299-L322`

_private_

```
async fn test_list_messages_for_session()
```

**Calls:** create_message, list_messages_for_session

### test_update_message_metrics_with_perf_data

*Rust Function* — `src/services/message.rs#L351-L384`

_private_

```
async fn test_update_message_metrics_with_perf_data()
```

**Calls:** create_message, update_message_metrics, get_message_required, from_str

### test_update_message_metrics_without_perf_data

*Rust Function* — `src/services/message.rs#L387-L409`

_private_

```
async fn test_update_message_metrics_without_perf_data()
```

**Calls:** create_message, update_message_metrics, get_message_required

### test_update_message_usage

*Rust Function* — `src/services/message.rs#L325-L348`

_private_

```
async fn test_update_message_usage()
```

**Calls:** create_message, update_message_usage, get_message_required

### begin_task

*Rust Method* — `src/services/plan.rs#L60-L64`

```
pub async fn begin_task(&self, task_id: Uuid) -> Result<()>
```

**Calls:** update_task_status

### complete_task

*Rust Method* — `src/services/plan.rs#L69-L73`

```
pub async fn complete_task(&self, task_id: Uuid, output_summary: Option<String>) -> Result<()>
```

**Calls:** update_task_status

### create

*Rust Method* — `src/services/plan.rs#L108-L110`

```
pub async fn create(&self, plan: &PlanDocument) -> Result<()>
```

### delete

*Rust Method* — `src/services/plan.rs#L118-L120`

```
pub async fn delete(&self, id: Uuid) -> Result<()>
```

### export_to_json

*Rust Method* — `src/services/plan.rs#L124-L137`

```
pub async fn export_to_json( &self, plan: &PlanDocument, file_path: &std::path::Path, ) -> Result<()>
```

**Called by:** test_service_export_to_json, test_service_export_import_roundtrip, test_service_atomic_json_write, save_plan, test_json_export_import_integration

### fail_task

*Rust Method* — `src/services/plan.rs#L76-L80`

```
pub async fn fail_task(&self, task_id: Uuid, error: String) -> Result<()>
```

**Calls:** update_task_status

### find_by_id

*Rust Method* — `src/services/plan.rs#L91-L93`

```
pub async fn find_by_id(&self, id: Uuid) -> Result<Option<PlanDocument>>
```

### find_by_session_id

*Rust Method* — `src/services/plan.rs#L96-L98`

```
pub async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<PlanDocument>>
```

### get_active_plans

*Rust Method* — `src/services/plan.rs#L256-L267`

```
pub async fn get_active_plans(&self, session_id: Uuid) -> Result<Vec<PlanDocument>>
```

### get_completed_plans

*Rust Method* — `src/services/plan.rs#L247-L253`

```
pub async fn get_completed_plans(&self, session_id: Uuid) -> Result<Vec<PlanDocument>>
```

### get_incomplete_tasks

*Rust Method* — `src/services/plan.rs#L83-L88`

```
pub async fn get_incomplete_tasks( &self, plan_id: Uuid, ) -> Result<Vec<crate::db::models::PlanTask>>
```

### get_most_recent_plan

*Rust Method* — `src/services/plan.rs#L101-L105`

```
pub async fn get_most_recent_plan(&self, session_id: Uuid) -> Result<Option<PlanDocument>>
```

**Calls:** next

**Called by:** test_service_get_most_recent_plan, load_plan_for_viewing, check_and_load_plan, test_get_most_recent_plan_integration

### get_plan_history

*Rust Method* — `src/services/plan.rs#L242-L244`

```
pub async fn get_plan_history(&self, session_id: Uuid) -> Result<Vec<PlanDocument>>
```

### get_statistics

*Rust Method* — `src/services/plan.rs#L270-L326`

```
pub async fn get_statistics(&self, session_id: Uuid) -> Result<PlanStatistics>
```

**Calls:** len, is_empty, progress_percentage

### import_from_json

*Rust Method* — `src/services/plan.rs#L141-L145`

```
pub async fn import_from_json(&self, file_path: &std::path::Path) -> Result<PlanDocument>
```

**Calls:** from_str

**Called by:** test_service_import_from_json, test_service_export_import_roundtrip, test_service_json_import_nonexistent_file, test_service_json_import_invalid_json, test_json_export_import_integration

### new

*Rust Method* — `src/services/plan.rs#L50-L55`

```
pub fn new(context: ServiceContext) -> Self
```

### update

*Rust Method* — `src/services/plan.rs#L113-L115`

```
pub async fn update(&self, plan: &PlanDocument) -> Result<()>
```

### validate_plan

*Rust Method* — `src/services/plan.rs#L148-L239`

```
pub fn validate_plan(&self, plan: &PlanDocument) -> Vec<PlanValidationWarning>
```

**Calls:** len, is_empty

### create_test_plan

*Rust Function* — `src/services/plan.rs#L365-L396`

_private_

```
fn create_test_plan(session_id: Uuid) -> PlanDocument
```

**Calls:** add_task

### setup_test_service

*Rust Function* — `src/services/plan.rs#L339-L362`

_private_

```
async fn setup_test_service() -> (Database, PlanService, Session, TempDir)
```

**Calls:** run_migrations

**Called by:** test_service_create_and_find, test_service_update, test_service_delete, test_service_find_by_session_id, test_service_get_most_recent_plan, test_service_export_to_json, test_service_import_from_json, test_service_export_import_roundtrip, test_service_atomic_json_write, test_service_json_import_nonexistent_file, test_service_json_import_invalid_json

### test_service_atomic_json_write

*Rust Function* — `src/services/plan.rs#L618-L636`

_private_

```
async fn test_service_atomic_json_write()
```

**Calls:** setup_test_service, export_to_json

### test_service_create_and_find

*Rust Function* — `src/services/plan.rs#L399-L415`

_private_

```
async fn test_service_create_and_find()
```

**Calls:** setup_test_service

### test_service_delete

*Rust Function* — `src/services/plan.rs#L441-L456`

_private_

```
async fn test_service_delete()
```

**Calls:** setup_test_service

### test_service_export_import_roundtrip

*Rust Function* — `src/services/plan.rs#L576-L615`

_private_

```
async fn test_service_export_import_roundtrip()
```

**Calls:** setup_test_service, export_to_json, import_from_json

### test_service_export_to_json

*Rust Function* — `src/services/plan.rs#L530-L551`

_private_

```
async fn test_service_export_to_json()
```

**Calls:** setup_test_service, export_to_json, from_str

### test_service_find_by_session_id

*Rust Function* — `src/services/plan.rs#L459-L482`

_private_

```
async fn test_service_find_by_session_id()
```

**Calls:** setup_test_service

### test_service_get_most_recent_plan

*Rust Function* — `src/services/plan.rs#L485-L527`

_private_

```
async fn test_service_get_most_recent_plan()
```

**Calls:** setup_test_service, get_most_recent_plan

### test_service_import_from_json

*Rust Function* — `src/services/plan.rs#L554-L573`

_private_

```
async fn test_service_import_from_json()
```

**Calls:** setup_test_service, import_from_json

### test_service_json_import_invalid_json

*Rust Function* — `src/services/plan.rs#L649-L657`

_private_

```
async fn test_service_json_import_invalid_json()
```

**Calls:** setup_test_service, import_from_json

### test_service_json_import_nonexistent_file

*Rust Function* — `src/services/plan.rs#L639-L646`

_private_

```
async fn test_service_json_import_nonexistent_file()
```

**Calls:** setup_test_service, import_from_json

### test_service_update

*Rust Function* — `src/services/plan.rs#L418-L438`

_private_

```
async fn test_service_update()
```

**Calls:** setup_test_service

### archive_session

*Rust Method* — `src/services/session.rs#L124-L132`

```
pub async fn archive_session(&self, id: Uuid) -> Result<()>
```

**Calls:** archive

**Called by:** test_archive_unarchive_session, test_count_sessions

### count_archived_sessions

*Rust Method* — `src/services/session.rs#L174-L179`

```
pub async fn count_archived_sessions(&self) -> Result<i64>
```

**Called by:** test_count_sessions

### count_sessions

*Rust Method* — `src/services/session.rs#L168-L171`

```
pub async fn count_sessions(&self) -> Result<i64>
```

**Called by:** test_count_sessions

### create_session

*Rust Method* — `src/services/session.rs#L29-L50`

```
pub async fn create_session(&self, title: Option<String>) -> Result<Session>
```

### delete_session

*Rust Method* — `src/services/session.rs#L146-L152`

```
pub async fn delete_session(&self, id: Uuid) -> Result<()>
```

**Called by:** test_delete_session

### end_session_with_summary

*Rust Method* — `src/services/session.rs#L186-L233`

```
pub async fn end_session_with_summary( &self, session_id: Uuid, messages: Vec<Message>, files_touched: Vec<String>, ) -> Result<()>
```

**Calls:** truncate_at_char_boundary, token_count

### get_most_recent_session

*Rust Method* — `src/services/session.rs#L155-L165`

```
pub async fn get_most_recent_session(&self) -> Result<Option<Session>>
```

**Calls:** list, next

**Called by:** test_get_most_recent_session, initialize

### get_session

*Rust Method* — `src/services/session.rs#L53-L56`

```
pub async fn get_session(&self, id: Uuid) -> Result<Option<Session>>
```

**Called by:** send_message_with_tools_inner, prepare_message_context, get_session_required, test_get_session, test_delete_session, load_session, test_error_database_concurrent_access, test_error_recovery_after_failure, test_end_to_end_cost_tracking, test_end_to_end_token_usage, test_database_persistence

### get_session_required

*Rust Method* — `src/services/session.rs#L59-L63`

```
pub async fn get_session_required(&self, id: Uuid) -> Result<Session>
```

**Calls:** get_session

**Called by:** update_session_title, update_session_usage, test_get_session_required, test_update_session_title, test_update_session_usage, test_archive_unarchive_session

### list_sessions

*Rust Method* — `src/services/session.rs#L66-L69`

```
pub async fn list_sessions(&self, options: SessionListOptions) -> Result<Vec<Session>>
```

**Calls:** list

**Called by:** test_list_sessions, load_sessions, test_end_to_end_session_management

### new

*Rust Method* — `src/services/session.rs#L24-L26`

```
pub fn new(context: ServiceContext) -> Self
```

### unarchive_session

*Rust Method* — `src/services/session.rs#L135-L143`

```
pub async fn unarchive_session(&self, id: Uuid) -> Result<()>
```

**Calls:** unarchive

**Called by:** test_archive_unarchive_session

### update_session

*Rust Method* — `src/services/session.rs#L72-L85`

```
pub async fn update_session(&self, session: &Session) -> Result<()>
```

**Called by:** create_new_session, load_session, complete_response, switch_provider_to_ollama_model

### update_session_title

*Rust Method* — `src/services/session.rs#L88-L100`

```
pub async fn update_session_title(&self, id: Uuid, title: Option<String>) -> Result<()>
```

**Calls:** get_session_required

**Called by:** test_update_session_title

### update_session_usage

*Rust Method* — `src/services/session.rs#L103-L121`

```
pub async fn update_session_usage(&self, id: Uuid, token_count: i32, cost: f64) -> Result<()>
```

**Calls:** get_session_required

**Called by:** send_message, send_message_with_tools_inner, test_update_session_usage

### create_test_service

*Rust Function* — `src/services/session.rs#L240-L249`

_private_

```
async fn create_test_service() -> SessionService
```

**Calls:** run_migrations

### test_archive_unarchive_session

*Rust Function* — `src/services/session.rs#L334-L350`

_private_

```
async fn test_archive_unarchive_session()
```

**Calls:** archive_session, get_session_required, unarchive_session

### test_count_sessions

*Rust Function* — `src/services/session.rs#L415-L439`

_private_

```
async fn test_count_sessions()
```

**Calls:** archive_session, count_sessions, count_archived_sessions

### test_create_session

*Rust Function* — `src/services/session.rs#L252-L263`

_private_

```
async fn test_create_session()
```

### test_delete_session

*Rust Function* — `src/services/session.rs#L353-L364`

_private_

```
async fn test_delete_session()
```

**Calls:** delete_session, get_session

### test_get_most_recent_session

*Rust Function* — `src/services/session.rs#L395-L412`

_private_

```
async fn test_get_most_recent_session()
```

**Calls:** get_most_recent_session

### test_get_session

*Rust Function* — `src/services/session.rs#L266-L276`

_private_

```
async fn test_get_session()
```

**Calls:** get_session

### test_get_session_required

*Rust Function* — `src/services/session.rs#L279-L292`

_private_

```
async fn test_get_session_required()
```

**Calls:** get_session_required

### test_list_sessions

*Rust Function* — `src/services/session.rs#L367-L392`

_private_

```
async fn test_list_sessions()
```

**Calls:** list_sessions

### test_update_session_title

*Rust Function* — `src/services/session.rs#L295-L309`

_private_

```
async fn test_update_session_title()
```

**Calls:** update_session_title, get_session_required

### test_update_session_usage

*Rust Function* — `src/services/session.rs#L312-L331`

_private_

```
async fn test_update_session_usage()
```

**Calls:** update_session_usage, get_session_required

### test_service_context_creation

*Rust Function* — `src/services/mod.rs#L103-L107`

_private_

```
async fn test_service_context_creation()
```

### test_service_manager_creation

*Rust Function* — `src/services/mod.rs#L110-L119`

_private_

```
async fn test_service_manager_creation()
```

**Calls:** sessions, messages, files

### append_streaming_chunk

*Rust Method* — `src/tui/app.rs#L1415-L1423`

_private_

```
fn append_streaming_chunk(&mut self, chunk: String)
```

**Called by:** handle_event

### auto_mode

*Rust Method* — `src/tui/app.rs#L495-L500`

```
pub fn auto_mode(&self) -> PlanExecMode
```

**Called by:** render_status_bar

### check_and_load_plan

*Rust Method* — `src/tui/app.rs#L1638-L1788`

_private_

```
async fn check_and_load_plan(&mut self) -> Result<()>
```

**Calls:** get_most_recent_plan, len

**Called by:** complete_response

### check_task_completion

*Rust Method* — `src/tui/app.rs#L1519-L1561`

_private_

```
async fn check_task_completion(&mut self, response_content: &str) -> Result<bool>
```

**Calls:** save_plan

**Called by:** complete_response

### clear_input

*Rust Method* — `src/tui/app.rs#L307-L309`

_private_

```
fn clear_input(&mut self)
```

**Calls:** plain_textarea

**Called by:** handle_chat_key, chat_input_text_is_not_underlined

### clear_session

*Rust Method* — `src/tui/app.rs#L1248-L1279`

_private_

```
async fn clear_session(&mut self) -> Result<()>
```

**Calls:** delete_messages_for_session

**Called by:** handle_key_event, clear_session_is_refused_while_the_current_session_is_processing, clear_session_proceeds_when_only_another_session_is_processing

### complete_response

*Rust Method* — `src/tui/app.rs#L1426-L1515`

_private_

```
async fn complete_response( &mut self, response: crate::llm::agent::AgentResponse, ) -> Result<()>
```

**Calls:** check_task_completion, tokens_per_second, update_session, execute_next_plan_task, check_and_load_plan

**Called by:** handle_event

### copy_last_response_to_clipboard

*Rust Method* — `src/tui/app.rs#L404-L415`

_private_

```
fn copy_last_response_to_clipboard(&mut self)
```

**Calls:** last_assistant_message, last_code_block

**Called by:** handle_chat_key

### create_new_session

*Rust Method* — `src/tui/app.rs#L1171-L1197`

_private_

```
async fn create_new_session(&mut self) -> Result<()>
```

**Calls:** update_session, sync_processing_state_for_current_session, load_sessions

**Called by:** initialize, handle_event, handle_key_event, stale_session_response_chunk_is_dropped_after_switching_sessions, plan_task_error_marks_task_failed_and_stops_auto_execution, stale_session_response_complete_is_dropped_after_switching_sessions, switching_sessions_clears_a_stuck_processing_state_from_the_previous_session, clear_session_is_refused_while_the_current_session_is_processing, clear_session_proceeds_when_only_another_session_is_processing, switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state, send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight, send_message_still_works_for_a_different_session_than_the_one_processing

### cursor_on_first_line

*Rust Method* — `src/tui/app.rs#L336-L338`

_private_

```
fn cursor_on_first_line(&self) -> bool
```

**Called by:** handle_chat_key

### cursor_on_last_line

*Rust Method* — `src/tui/app.rs#L340-L342`

_private_

```
fn cursor_on_last_line(&self) -> bool
```

**Calls:** len

**Called by:** handle_chat_key

### cycle_auto_mode

*Rust Method* — `src/tui/app.rs#L503-L510`

_private_

```
fn cycle_auto_mode(&mut self)
```

**Called by:** handle_key_event

### event_belongs_to_current_session

*Rust Method* — `src/tui/app.rs#L1381-L1385`

_private_

```
fn event_belongs_to_current_session(&self, session_id: Uuid) -> bool
```

**Called by:** handle_event

### event_handler

*Rust Method* — `src/tui/app.rs#L448-L450`

```
pub fn event_handler(&self) -> &EventHandler
```

### event_handler_mut

*Rust Method* — `src/tui/app.rs#L453-L455`

```
pub fn event_handler_mut(&mut self) -> &mut EventHandler
```

### event_sender

*Rust Method* — `src/tui/app.rs#L458-L460`

```
pub fn event_sender(&self) -> tokio::sync::mpsc::UnboundedSender<TuiEvent>
```

**Calls:** sender

**Called by:** cmd_chat, handle_event, send_message, handle_approval_key, open_model_download, start_model_pull, start_model_delete, open_provider_switch, run_inner

### execute_next_plan_task

*Rust Method* — `src/tui/app.rs#L1918-L2018`

_private_

```
async fn execute_next_plan_task(&mut self) -> Result<()>
```

**Calls:** tasks_in_order, show_error, len, drop, save_plan

**Called by:** complete_response, execute_plan_tasks

### execute_plan_tasks

*Rust Method* — `src/tui/app.rs#L1912-L1915`

_private_

```
async fn execute_plan_tasks(&mut self) -> Result<()>
```

**Calls:** execute_next_plan_task

**Called by:** handle_plan_key

### export_plan_to_markdown

*Rust Method* — `src/tui/app.rs#L1793-L1870`

_private_

```
async fn export_plan_to_markdown(&self, filename: &str) -> Result<()>
```

**Calls:** is_empty

**Called by:** handle_plan_key

### fail_current_plan_task

*Rust Method* — `src/tui/app.rs#L2029-L2051`

_private_

```
async fn fail_current_plan_task(&mut self, error: &str) -> Result<()>
```

**Calls:** save_plan

**Called by:** handle_event

### handle_approval_key

*Rust Method* — `src/tui/app.rs#L2093-L2134`

_private_

```
async fn handle_approval_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>
```

**Calls:** is_approve, event_sender, is_deny, is_cancel, is_view_details

**Called by:** handle_key_event

### handle_approval_requested

*Rust Method* — `src/tui/app.rs#L2086-L2090`

_private_

```
fn handle_approval_requested(&mut self, request: ToolApprovalRequest)
```

**Called by:** handle_event

### handle_chat_key

*Rust Method* — `src/tui/app.rs#L861-L969`

_private_

```
async fn handle_chat_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>
```

**Calls:** is_submit, input_is_blank, input_text, clear_input, push_input_history, try_handle_slash_command, is_newline, is_cancel, is_page_up, is_page_down, is_copy_response, copy_last_response_to_clipboard, is_paste_clipboard, paste_from_clipboard, open_file_picker, is_empty, cursor_on_first_line, history_prev, cursor_on_last_line, history_next

**Called by:** handle_key_event, up_recalls_previous_messages_without_sending_them, recalled_message_can_be_edited_before_resending, up_moves_the_cursor_inside_a_multiline_draft, up_is_plain_cursor_movement_when_there_is_no_history, consecutive_duplicate_submissions_are_stored_once, chat_shift_enter_inserts_newline_instead_of_submitting, chat_alt_enter_inserts_newline_as_non_kitty_fallback, chat_left_arrow_moves_cursor_for_mid_buffer_insert, chat_backspace_deletes_at_cursor_not_always_the_last_char, chat_home_and_end_move_cursor_to_line_boundaries, chat_ctrl_left_right_jump_by_word, chat_ctrl_backspace_deletes_whole_word, altgr_backslash_reaches_the_input, altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut, plain_at_sign_still_opens_the_file_picker, typed_backslashes_reach_the_input, paste_inserts_at_cursor_not_always_appended_at_the_end, ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard, ctrl_y_copies_last_code_block_when_present, ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking, typing_and_submitting_slash_skills_opens_the_dialog_end_to_end, chat_plain_enter_submits_and_clears_buffer, chat_plain_enter_on_empty_buffer_does_nothing, chat_ctrl_enter_still_submits_as_legacy_alias

### handle_event

*Rust Method* — `src/tui/app.rs#L529-L746`

```
pub async fn handle_event(&mut self, event: TuiEvent) -> Result<()>
```

**Calls:** handle_key_event, event_belongs_to_current_session, append_streaming_chunk, complete_response, fail_current_plan_task, show_error, switch_mode, load_session, create_new_session, is_timed_out, event_sender, handle_approval_requested, refresh_model_download_suggestions

**Called by:** handle_ollama_models_listed_updates_installed_list, handle_ollama_pull_progress_updates_status_and_fraction, handle_ollama_pull_finished_success_posts_chat_message, handle_ollama_pull_finished_failure_posts_error_message, delete_key_on_installed_model_asks_for_confirmation, handle_ollama_delete_finished_success_removes_from_installed_and_posts_message, handle_ollama_delete_finished_failure_keeps_installed_and_posts_error, key_release_events_are_ignored, paste_preserves_backslashes_and_newlines, paste_inserts_at_cursor_not_always_appended_at_the_end, paste_with_embedded_newline_produces_multiple_lines, provider_switch_models_listed_clears_loading_state, stale_session_response_chunk_is_dropped_after_switching_sessions, plan_task_error_marks_task_failed_and_stops_auto_execution, stale_session_response_complete_is_dropped_after_switching_sessions, run_loop

### handle_file_picker_key

*Rust Method* — `src/tui/app.rs#L2173-L2224`

_private_

```
async fn handle_file_picker_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>
```

**Calls:** is_cancel, switch_mode, is_up, is_down, len, is_enter, open_file_picker

**Called by:** handle_key_event

### handle_key_event

*Rust Method* — `src/tui/app.rs#L749-L858`

_private_

```
async fn handle_key_event(&mut self, event: crossterm::event::KeyEvent) -> Result<()>
```

**Calls:** is_quit, is_new_session, create_new_session, is_list_sessions, switch_mode, is_help, is_clear_session, clear_session, is_toggle_plan, load_plan_for_viewing, is_toggle_auto_mode, cycle_auto_mode, is_model_download, open_model_download, is_model_info, is_provider_switch, open_provider_switch, handle_chat_key, handle_plan_key, handle_sessions_key, handle_approval_key, handle_file_picker_key, handle_model_download_key, handle_provider_switch_key, handle_skills_key, handle_mcp_key, is_cancel

**Called by:** handle_event, shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps, shift_tab_works_from_any_mode_not_just_chat, setting_auto_mode_state_shares_the_same_cell_as_a_clone, ctrl_o_opens_model_info_panel_and_esc_closes_it, ctrl_w_opens_provider_switch_dialog_in_loading_state

### handle_mcp_key

*Rust Method* — `src/tui/app.rs#L1027-L1039`

_private_

```
async fn handle_mcp_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>
```

**Calls:** is_cancel, switch_mode, is_up, is_down, is_empty, len

**Called by:** handle_key_event, mcp_view_up_down_navigation_clamps_at_bounds, mcp_view_esc_returns_to_chat

### handle_model_download_key

*Rust Method* — `src/tui/app.rs#L2291-L2379`

_private_

```
async fn handle_model_download_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>
```

**Calls:** start_model_delete, is_cancel, switch_mode, is_up, is_down, is_empty, len, refresh_model_download_suggestions, is_enter, start_model_pull

**Called by:** handle_key_event, model_download_typing_filters_suggestions, model_download_backspace_removes_last_char, model_download_tab_adopts_highlighted_suggestion, model_download_esc_closes_dialog_without_running_pull, model_download_enter_starts_pull_then_esc_aborts_it, delete_key_ignored_for_uninstalled_suggestion, delete_key_on_installed_model_asks_for_confirmation, confirm_delete_n_cancels_back_to_list, confirm_delete_esc_cancels_back_to_list_without_closing_dialog, confirm_delete_y_starts_delete

### handle_plan_key

*Rust Method* — `src/tui/app.rs#L1070-L1168`

_private_

```
async fn handle_plan_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>
```

**Calls:** is_cancel, switch_mode, export_plan_to_markdown, save_plan, execute_plan_tasks, reject, set_input_text, len

**Called by:** handle_key_event

### handle_provider_switch_key

*Rust Method* — `src/tui/app.rs#L2400-L2435`

_private_

```
async fn handle_provider_switch_key( &mut self, event: crossterm::event::KeyEvent, ) -> Result<()>
```

**Calls:** is_cancel, switch_mode, is_up, is_down, is_empty, len, is_enter, switch_provider_to_ollama_model

**Called by:** handle_key_event, provider_switch_up_down_navigation_clamps_at_bounds, provider_switch_esc_returns_to_chat

### handle_sessions_key

*Rust Method* — `src/tui/app.rs#L972-L990`

_private_

```
async fn handle_sessions_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>
```

**Calls:** is_cancel, switch_mode, is_up, is_down, len, is_enter, load_session

**Called by:** handle_key_event

### handle_skills_key

*Rust Method* — `src/tui/app.rs#L1003-L1015`

_private_

```
async fn handle_skills_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>
```

**Calls:** is_cancel, switch_mode, is_up, is_down, is_empty, len

**Called by:** handle_key_event, skills_view_up_down_navigation_clamps_at_bounds, skills_view_esc_returns_to_chat

### history_next

*Rust Method* — `src/tui/app.rs#L381-L397`

_private_

```
fn history_next(&mut self) -> bool
```

**Calls:** len, load_history_entry

**Called by:** handle_chat_key

### history_prev

*Rust Method* — `src/tui/app.rs#L356-L375`

_private_

```
fn history_prev(&mut self) -> bool
```

**Calls:** is_empty, input_text, len, load_history_entry

**Called by:** handle_chat_key

### initialize

*Rust Method* — `src/tui/app.rs#L432-L445`

```
pub async fn initialize(&mut self) -> Result<()>
```

**Calls:** get_most_recent_session, load_session, create_new_session, load_sessions

**Called by:** run_inner

### input_is_blank

*Rust Method* — `src/tui/app.rs#L299-L304`

_private_

```
fn input_is_blank(&self) -> bool
```

**Calls:** is_empty

**Called by:** handle_chat_key

### input_text

*Rust Method* — `src/tui/app.rs#L292-L294`

```
pub fn input_text(&self) -> String
```

**Called by:** history_prev, handle_chat_key

### last_assistant_message

*Rust Method* — `src/tui/app.rs#L287-L289`

```
pub fn last_assistant_message(&self) -> Option<&DisplayMessage>
```

**Called by:** copy_last_response_to_clipboard, render_model_info

### load_history_entry

*Rust Method* — `src/tui/app.rs#L346-L350`

_private_

```
fn load_history_entry(&mut self, entry: &str)
```

**Calls:** set_input_text

**Called by:** history_prev, history_next

### load_plan_for_viewing

*Rust Method* — `src/tui/app.rs#L1565-L1633`

_private_

```
async fn load_plan_for_viewing(&mut self) -> Result<()>
```

**Calls:** get_most_recent_plan

**Called by:** handle_key_event

### load_session

*Rust Method* — `src/tui/app.rs#L1200-L1229`

_private_

```
async fn load_session(&mut self, session_id: Uuid) -> Result<()>
```

**Calls:** get_session, list_messages_for_session, update_session, sync_processing_state_for_current_session

**Called by:** initialize, handle_event, handle_sessions_key, switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state

### load_sessions

*Rust Method* — `src/tui/app.rs#L1232-L1245`

_private_

```
async fn load_sessions(&mut self) -> Result<()>
```

**Calls:** list_sessions

**Called by:** initialize, create_new_session, switch_mode

### new

*Rust Method* — `src/tui/app.rs#L208-L267`

```
pub fn new(agent_service: Arc<AgentService>, context: ServiceContext) -> Self
```

**Calls:** plain_textarea

### next_event

*Rust Method* — `src/tui/app.rs#L519-L521`

```
pub async fn next_event(&mut self) -> Option<TuiEvent>
```

**Calls:** next

**Called by:** run_loop

### open_file_picker

*Rust Method* — `src/tui/app.rs#L2137-L2170`

_private_

```
async fn open_file_picker(&mut self) -> Result<()>
```

**Calls:** switch_mode

**Called by:** handle_chat_key, handle_file_picker_key

### open_mcp

*Rust Method* — `src/tui/app.rs#L1021-L1024`

_private_

```
async fn open_mcp(&mut self) -> Result<()>
```

**Calls:** switch_mode

**Called by:** try_handle_slash_command

### open_model_download

*Rust Method* — `src/tui/app.rs#L2229-L2247`

_private_

```
async fn open_model_download(&mut self) -> Result<()>
```

**Calls:** refresh_model_download_suggestions, event_sender, switch_mode

**Called by:** handle_key_event, open_model_download_switches_mode_and_seeds_suggestions, model_download_typing_filters_suggestions, model_download_backspace_removes_last_char, model_download_tab_adopts_highlighted_suggestion, model_download_esc_closes_dialog_without_running_pull, model_download_enter_starts_pull_then_esc_aborts_it, delete_key_ignored_for_uninstalled_suggestion, delete_key_on_installed_model_asks_for_confirmation, confirm_delete_n_cancels_back_to_list, confirm_delete_esc_cancels_back_to_list_without_closing_dialog, confirm_delete_y_starts_delete

### open_provider_switch

*Rust Method* — `src/tui/app.rs#L2384-L2397`

_private_

```
async fn open_provider_switch(&mut self) -> Result<()>
```

**Calls:** event_sender, switch_mode

**Called by:** handle_key_event

### open_skills

*Rust Method* — `src/tui/app.rs#L996-L1000`

_private_

```
async fn open_skills(&mut self) -> Result<()>
```

**Calls:** list_skills, switch_mode

**Called by:** try_handle_slash_command

### paste_from_clipboard

*Rust Method* — `src/tui/app.rs#L420-L429`

_private_

```
fn paste_from_clipboard(&mut self)
```

**Called by:** handle_chat_key

### provider_context_window

*Rust Method* — `src/tui/app.rs#L281-L283`

```
pub fn provider_context_window(&self) -> Option<u32>
```

### provider_model

*Rust Method* — `src/tui/app.rs#L275-L277`

```
pub fn provider_model(&self) -> &str
```

### provider_name

*Rust Method* — `src/tui/app.rs#L270-L272`

```
pub fn provider_name(&self) -> &str
```

### push_input_history

*Rust Method* — `src/tui/app.rs#L323-L330`

_private_

```
fn push_input_history(&mut self, content: &str)
```

**Called by:** handle_chat_key, up_recalls_previous_messages_without_sending_them, key_release_events_are_ignored, recalled_message_can_be_edited_before_resending, up_moves_the_cursor_inside_a_multiline_draft, consecutive_duplicate_submissions_are_stored_once

### refresh_model_download_suggestions

*Rust Method* — `src/tui/app.rs#L2250-L2256`

_private_

```
fn refresh_model_download_suggestions(&mut self)
```

**Calls:** filter_suggestions

**Called by:** handle_event, open_model_download, handle_model_download_key

### save_plan

*Rust Method* — `src/tui/app.rs#L1872-L1909`

_private_

```
async fn save_plan(&self) -> Result<()>
```

**Calls:** export_to_json

**Called by:** handle_plan_key, check_task_completion, execute_next_plan_task, fail_current_plan_task

### send_message

*Rust Method* — `src/tui/app.rs#L1282-L1371`

_private_

```
async fn send_message(&mut self, content: String) -> Result<()>
```

**Calls:** analyze_and_transform, event_sender, send_message_with_tools_and_mode_streaming

### set_agent_service

*Rust Method* — `src/tui/app.rs#L463-L465`

```
pub fn set_agent_service(&mut self, agent_service: Arc<AgentService>)
```

**Called by:** cmd_chat

### set_auto_mode_state

*Rust Method* — `src/tui/app.rs#L490-L492`

```
pub fn set_auto_mode_state(&mut self, auto_mode: Arc<Mutex<PlanExecMode>>)
```

**Called by:** cmd_chat, setting_auto_mode_state_shares_the_same_cell_as_a_clone, status_bar_shows_full_auto_when_active

### set_input_text

*Rust Method* — `src/tui/app.rs#L314-L317`

_private_

```
fn set_input_text(&mut self, text: &str)
```

**Calls:** plain_textarea

**Called by:** load_history_entry, handle_plan_key, up_recalls_previous_messages_without_sending_them, up_moves_the_cursor_inside_a_multiline_draft, up_is_plain_cursor_movement_when_there_is_no_history, chat_input_text_is_not_underlined

### set_kitty_keyboard_protocol_active

*Rust Method* — `src/tui/app.rs#L482-L484`

```
pub fn set_kitty_keyboard_protocol_active(&mut self, active: bool)
```

**Called by:** run

### set_mcp_status

*Rust Method* — `src/tui/app.rs#L514-L516`

```
pub fn set_mcp_status(&mut self, status: Vec<crate::mcp::McpServerStatus>)
```

**Called by:** cmd_chat

### set_ollama_config

*Rust Method* — `src/tui/app.rs#L476-L478`

```
pub fn set_ollama_config(&mut self, config: crate::config::OllamaProviderConfig)
```

**Called by:** cmd_chat

### set_ollama_host

*Rust Method* — `src/tui/app.rs#L469-L471`

```
pub fn set_ollama_host(&mut self, host: String)
```

**Called by:** cmd_chat

### show_error

*Rust Method* — `src/tui/app.rs#L2054-L2061`

_private_

```
fn show_error(&mut self, error: String)
```

**Called by:** handle_event, execute_next_plan_task

### start_model_delete

*Rust Method* — `src/tui/app.rs#L2277-L2288`

_private_

```
async fn start_model_delete(&mut self, model: String)
```

**Calls:** event_sender

**Called by:** handle_model_download_key

### start_model_pull

*Rust Method* — `src/tui/app.rs#L2260-L2273`

_private_

```
async fn start_model_pull(&mut self, model: String)
```

**Calls:** is_empty, event_sender

**Called by:** handle_model_download_key

### switch_mode

*Rust Method* — `src/tui/app.rs#L2064-L2073`

_private_

```
async fn switch_mode(&mut self, mode: AppMode) -> Result<()>
```

**Calls:** load_sessions

**Called by:** handle_event, handle_key_event, handle_sessions_key, open_skills, handle_skills_key, open_mcp, handle_mcp_key, try_handle_slash_command, handle_plan_key, open_file_picker, handle_file_picker_key, open_model_download, handle_model_download_key, open_provider_switch, handle_provider_switch_key, switch_provider_to_ollama_model

### switch_provider_to_ollama_model

*Rust Method* — `src/tui/app.rs#L2447-L2497`

_private_

```
async fn switch_provider_to_ollama_model(&mut self, model: String) -> Result<()>
```

**Calls:** set_provider, update_session, switch_mode

**Called by:** handle_provider_switch_key, switch_provider_without_ollama_feature_shows_clear_error, switch_provider_with_ollama_feature_swaps_provider_in_place

### sync_processing_state_for_current_session

*Rust Method* — `src/tui/app.rs#L1406-L1412`

_private_

```
fn sync_processing_state_for_current_session(&mut self)
```

**Called by:** create_new_session, load_session

### total_cost

*Rust Method* — `src/tui/app.rs#L2081-L2083`

```
pub fn total_cost(&self) -> f64
```

**Called by:** render_header

### total_tokens

*Rust Method* — `src/tui/app.rs#L2076-L2078`

```
pub fn total_tokens(&self) -> i32
```

**Called by:** render_header

### try_handle_slash_command

*Rust Method* — `src/tui/app.rs#L1046-L1067`

_private_

```
async fn try_handle_slash_command(&mut self, content: &str) -> Result<bool>
```

**Calls:** next, open_skills, open_mcp, switch_mode

**Called by:** handle_chat_key, unrecognized_slash_word_falls_through_instead_of_being_swallowed, non_slash_message_is_never_treated_as_a_command

### try_next_event

*Rust Method* — `src/tui/app.rs#L524-L526`

```
pub fn try_next_event(&mut self) -> Option<TuiEvent>
```

**Calls:** try_next

**Called by:** run_loop

### from

*Rust Method* — `src/tui/app.rs#L44-L63`

_private_

```
fn from(msg: Message) -> Self
```

**Calls:** from_str

### calculate_cost

*Rust Method* — `src/tui/app.rs#L2554-L2556`

_private_

```
fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64
```

### complete

*Rust Method* — `src/tui/app.rs#L2536-L2538`

_private_

```
async fn complete(&self, _request: LLMRequest) -> ProviderResult<LLMResponse>
```

### context_window

*Rust Method* — `src/tui/app.rs#L2551-L2553`

_private_

```
fn context_window(&self, _model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/tui/app.rs#L2545-L2547`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/tui/app.rs#L2542-L2544`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/tui/app.rs#L2539-L2541`

_private_

```
async fn stream(&self, _request: LLMRequest) -> ProviderResult<ProviderStream>
```

### supported_models

*Rust Method* — `src/tui/app.rs#L2548-L2550`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut

*Rust Function* — `src/tui/app.rs#L3170-L3183`

_private_

```
async fn altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut()
```

**Calls:** handle_chat_key, key_mod

### altgr_backslash_reaches_the_input

*Rust Function* — `src/tui/app.rs#L3146-L3165`

_private_

```
async fn altgr_backslash_reaches_the_input()
```

**Calls:** key_mod, key, handle_chat_key

### auto_mode_defaults_to_interactive

*Rust Function* — `src/tui/app.rs#L3313-L3316`

_private_

```
async fn auto_mode_defaults_to_interactive()
```

### chat_alt_enter_inserts_newline_as_non_kitty_fallback

*Rust Function* — `src/tui/app.rs#L3034-L3042`

_private_

```
async fn chat_alt_enter_inserts_newline_as_non_kitty_fallback()
```

**Calls:** handle_chat_key, key, key_mod

### chat_backspace_deletes_at_cursor_not_always_the_last_char

*Rust Function* — `src/tui/app.rs#L3060-L3073`

_private_

```
async fn chat_backspace_deletes_at_cursor_not_always_the_last_char()
```

**Calls:** handle_chat_key, key

### chat_ctrl_backspace_deletes_whole_word

*Rust Function* — `src/tui/app.rs#L3107-L3117`

_private_

```
async fn chat_ctrl_backspace_deletes_whole_word()
```

**Calls:** handle_chat_key, key, key_mod

### chat_ctrl_enter_still_submits_as_legacy_alias

*Rust Function* — `src/tui/app.rs#L3635-L3644`

_private_

```
async fn chat_ctrl_enter_still_submits_as_legacy_alias()
```

**Calls:** handle_chat_key, key, key_mod

### chat_ctrl_left_right_jump_by_word

*Rust Function* — `src/tui/app.rs#L3091-L3104`

_private_

```
async fn chat_ctrl_left_right_jump_by_word()
```

**Calls:** handle_chat_key, key, key_mod

### chat_home_and_end_move_cursor_to_line_boundaries

*Rust Function* — `src/tui/app.rs#L3076-L3088`

_private_

```
async fn chat_home_and_end_move_cursor_to_line_boundaries()
```

**Calls:** handle_chat_key, key

### chat_input_text_is_not_underlined

*Rust Function* — `src/tui/app.rs#L3421-L3444`

_private_

```
async fn chat_input_text_is_not_underlined()
```

**Calls:** clear_input, set_input_text

### chat_left_arrow_moves_cursor_for_mid_buffer_insert

*Rust Function* — `src/tui/app.rs#L3045-L3057`

_private_

```
async fn chat_left_arrow_moves_cursor_for_mid_buffer_insert()
```

**Calls:** handle_chat_key, key

### chat_plain_enter_on_empty_buffer_does_nothing

*Rust Function* — `src/tui/app.rs#L3563-L3570`

_private_

```
async fn chat_plain_enter_on_empty_buffer_does_nothing()
```

**Calls:** handle_chat_key, key

### chat_plain_enter_submits_and_clears_buffer

*Rust Function* — `src/tui/app.rs#L3549-L3560`

_private_

```
async fn chat_plain_enter_submits_and_clears_buffer()
```

**Calls:** handle_chat_key, key

### chat_shift_enter_inserts_newline_instead_of_submitting

*Rust Function* — `src/tui/app.rs#L3019-L3031`

_private_

```
async fn chat_shift_enter_inserts_newline_instead_of_submitting()
```

**Calls:** handle_chat_key, key, key_mod

### clear_session_is_refused_while_the_current_session_is_processing

*Rust Function* — `src/tui/app.rs#L3948-L3981`

_private_

```
async fn clear_session_is_refused_while_the_current_session_is_processing()
```

**Calls:** create_new_session, create_message, clear_session, list_messages_for_session

### clear_session_proceeds_when_only_another_session_is_processing

*Rust Function* — `src/tui/app.rs#L3986-L4019`

_private_

```
async fn clear_session_proceeds_when_only_another_session_is_processing()
```

**Calls:** create_new_session, create_message, clear_session, list_messages_for_session

### confirm_delete_esc_cancels_back_to_list_without_closing_dialog

*Rust Function* — `src/tui/app.rs#L2794-L2809`

_private_

```
async fn confirm_delete_esc_cancels_back_to_list_without_closing_dialog()
```

**Calls:** open_model_download, handle_model_download_key, key

### confirm_delete_n_cancels_back_to_list

*Rust Function* — `src/tui/app.rs#L2779-L2791`

_private_

```
async fn confirm_delete_n_cancels_back_to_list()
```

**Calls:** open_model_download, handle_model_download_key, key

### confirm_delete_y_starts_delete

*Rust Function* — `src/tui/app.rs#L2812-L2826`

_private_

```
async fn confirm_delete_y_starts_delete()
```

**Calls:** open_model_download, handle_model_download_key, key

### consecutive_duplicate_submissions_are_stored_once

*Rust Function* — `src/tui/app.rs#L2996-L3012`

_private_

```
async fn consecutive_duplicate_submissions_are_stored_once()
```

**Calls:** push_input_history, handle_chat_key, key

### ctrl_o_opens_model_info_panel_and_esc_closes_it

*Rust Function* — `src/tui/app.rs#L3573-L3584`

_private_

```
async fn ctrl_o_opens_model_info_panel_and_esc_closes_it()
```

**Calls:** handle_key_event, key_mod, key

### ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking

*Rust Function* — `src/tui/app.rs#L3291-L3310`

_private_

```
async fn ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking()
```

**Calls:** handle_chat_key, key_mod

### ctrl_w_opens_provider_switch_dialog_in_loading_state

*Rust Function* — `src/tui/app.rs#L3647-L3658`

_private_

```
async fn ctrl_w_opens_provider_switch_dialog_in_loading_state()
```

**Calls:** handle_key_event, key_mod

### ctrl_y_copies_last_code_block_when_present

*Rust Function* — `src/tui/app.rs#L3253-L3288`

_private_

```
async fn ctrl_y_copies_last_code_block_when_present()
```

**Calls:** handle_chat_key, key_mod

### ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard

*Rust Function* — `src/tui/app.rs#L3238-L3250`

_private_

```
async fn ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard()
```

**Calls:** handle_chat_key, key_mod

### delete_key_ignored_for_uninstalled_suggestion

*Rust Function* — `src/tui/app.rs#L2739-L2750`

_private_

```
async fn delete_key_ignored_for_uninstalled_suggestion()
```

**Calls:** open_model_download, handle_model_download_key, key

### delete_key_on_installed_model_asks_for_confirmation

*Rust Function* — `src/tui/app.rs#L2753-L2776`

_private_

```
async fn delete_key_on_installed_model_asks_for_confirmation()
```

**Calls:** open_model_download, handle_event, handle_model_download_key, key

### handle_ollama_delete_finished_failure_keeps_installed_and_posts_error

*Rust Function* — `src/tui/app.rs#L2854-L2874`

_private_

```
async fn handle_ollama_delete_finished_failure_keeps_installed_and_posts_error()
```

**Calls:** handle_event

### handle_ollama_delete_finished_success_removes_from_installed_and_posts_message

*Rust Function* — `src/tui/app.rs#L2829-L2851`

_private_

```
async fn handle_ollama_delete_finished_success_removes_from_installed_and_posts_message()
```

**Calls:** handle_event

### handle_ollama_models_listed_updates_installed_list

*Rust Function* — `src/tui/app.rs#L2663-L2675`

_private_

```
async fn handle_ollama_models_listed_updates_installed_list()
```

**Calls:** handle_event

### handle_ollama_pull_finished_failure_posts_error_message

*Rust Function* — `src/tui/app.rs#L2719-L2734`

_private_

```
async fn handle_ollama_pull_finished_failure_posts_error_message()
```

**Calls:** handle_event

### handle_ollama_pull_finished_success_posts_chat_message

*Rust Function* — `src/tui/app.rs#L2698-L2716`

_private_

```
async fn handle_ollama_pull_finished_success_posts_chat_message()
```

**Calls:** handle_event

### handle_ollama_pull_progress_updates_status_and_fraction

*Rust Function* — `src/tui/app.rs#L2678-L2695`

_private_

```
async fn handle_ollama_pull_progress_updates_status_and_fraction()
```

**Calls:** handle_event

### key

*Rust Function* — `src/tui/app.rs#L2567-L2569`

_private_

```
fn key(code: KeyCode) -> crossterm::event::KeyEvent
```

**Calls:** empty

**Called by:** model_download_typing_filters_suggestions, model_download_backspace_removes_last_char, model_download_tab_adopts_highlighted_suggestion, model_download_esc_closes_dialog_without_running_pull, model_download_enter_starts_pull_then_esc_aborts_it, delete_key_ignored_for_uninstalled_suggestion, delete_key_on_installed_model_asks_for_confirmation, confirm_delete_n_cancels_back_to_list, confirm_delete_esc_cancels_back_to_list_without_closing_dialog, confirm_delete_y_starts_delete, up_recalls_previous_messages_without_sending_them, recalled_message_can_be_edited_before_resending, up_moves_the_cursor_inside_a_multiline_draft, up_is_plain_cursor_movement_when_there_is_no_history, consecutive_duplicate_submissions_are_stored_once, chat_shift_enter_inserts_newline_instead_of_submitting, chat_alt_enter_inserts_newline_as_non_kitty_fallback, chat_left_arrow_moves_cursor_for_mid_buffer_insert, chat_backspace_deletes_at_cursor_not_always_the_last_char, chat_home_and_end_move_cursor_to_line_boundaries, chat_ctrl_left_right_jump_by_word, chat_ctrl_backspace_deletes_whole_word, altgr_backslash_reaches_the_input, plain_at_sign_still_opens_the_file_picker, typed_backslashes_reach_the_input, paste_inserts_at_cursor_not_always_appended_at_the_end, shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps, shift_tab_works_from_any_mode_not_just_chat, setting_auto_mode_state_shares_the_same_cell_as_a_clone, typing_and_submitting_slash_skills_opens_the_dialog_end_to_end, skills_view_up_down_navigation_clamps_at_bounds, skills_view_esc_returns_to_chat, mcp_view_up_down_navigation_clamps_at_bounds, mcp_view_esc_returns_to_chat, chat_plain_enter_submits_and_clears_buffer, chat_plain_enter_on_empty_buffer_does_nothing, ctrl_o_opens_model_info_panel_and_esc_closes_it, chat_ctrl_enter_still_submits_as_legacy_alias, provider_switch_up_down_navigation_clamps_at_bounds, provider_switch_esc_returns_to_chat

### key_mod

*Rust Function* — `src/tui/app.rs#L3014-L3016`

_private_

```
fn key_mod(code: KeyCode, modifiers: KeyModifiers) -> crossterm::event::KeyEvent
```

**Called by:** chat_shift_enter_inserts_newline_instead_of_submitting, chat_alt_enter_inserts_newline_as_non_kitty_fallback, chat_ctrl_left_right_jump_by_word, chat_ctrl_backspace_deletes_whole_word, altgr_backslash_reaches_the_input, altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut, ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard, ctrl_y_copies_last_code_block_when_present, ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking, ctrl_o_opens_model_info_panel_and_esc_closes_it, chat_ctrl_enter_still_submits_as_legacy_alias, ctrl_w_opens_provider_switch_dialog_in_loading_state

### key_release_events_are_ignored

*Rust Function* — `src/tui/app.rs#L2915-L2943`

_private_

```
async fn key_release_events_are_ignored()
```

**Calls:** push_input_history, handle_event

### last_assistant_message_finds_most_recent_assistant_reply

*Rust Function* — `src/tui/app.rs#L3587-L3632`

_private_

```
async fn last_assistant_message_finds_most_recent_assistant_reply()
```

### mcp_view_esc_returns_to_chat

*Rust Function* — `src/tui/app.rs#L3539-L3546`

_private_

```
async fn mcp_view_esc_returns_to_chat()
```

**Calls:** handle_mcp_key, key

### mcp_view_up_down_navigation_clamps_at_bounds

*Rust Function* — `src/tui/app.rs#L3510-L3536`

_private_

```
async fn mcp_view_up_down_navigation_clamps_at_bounds()
```

**Calls:** handle_mcp_key, key

### model_download_backspace_removes_last_char

*Rust Function* — `src/tui/app.rs#L2604-L2614`

_private_

```
async fn model_download_backspace_removes_last_char()
```

**Calls:** open_model_download, handle_model_download_key, key

### model_download_enter_starts_pull_then_esc_aborts_it

*Rust Function* — `src/tui/app.rs#L2643-L2660`

_private_

```
async fn model_download_enter_starts_pull_then_esc_aborts_it()
```

**Calls:** open_model_download, handle_model_download_key, key

### model_download_esc_closes_dialog_without_running_pull

*Rust Function* — `src/tui/app.rs#L2630-L2640`

_private_

```
async fn model_download_esc_closes_dialog_without_running_pull()
```

**Calls:** open_model_download, handle_model_download_key, key

### model_download_tab_adopts_highlighted_suggestion

*Rust Function* — `src/tui/app.rs#L2617-L2627`

_private_

```
async fn model_download_tab_adopts_highlighted_suggestion()
```

**Calls:** open_model_download, handle_model_download_key, key

### model_download_typing_filters_suggestions

*Rust Function* — `src/tui/app.rs#L2585-L2601`

_private_

```
async fn model_download_typing_filters_suggestions()
```

**Calls:** open_model_download, handle_model_download_key, key

### non_slash_message_is_never_treated_as_a_command

*Rust Function* — `src/tui/app.rs#L3447-L3454`

_private_

```
async fn non_slash_message_is_never_treated_as_a_command()
```

**Calls:** try_handle_slash_command

### open_model_download_switches_mode_and_seeds_suggestions

*Rust Function* — `src/tui/app.rs#L2572-L2582`

_private_

```
async fn open_model_download_switches_mode_and_seeds_suggestions()
```

**Calls:** open_model_download

### paste_inserts_at_cursor_not_always_appended_at_the_end

*Rust Function* — `src/tui/app.rs#L3210-L3223`

_private_

```
async fn paste_inserts_at_cursor_not_always_appended_at_the_end()
```

**Calls:** handle_chat_key, key, handle_event

### paste_preserves_backslashes_and_newlines

*Rust Function* — `src/tui/app.rs#L3123-L3137`

_private_

```
async fn paste_preserves_backslashes_and_newlines()
```

**Calls:** handle_event

### paste_with_embedded_newline_produces_multiple_lines

*Rust Function* — `src/tui/app.rs#L3226-L3235`

_private_

```
async fn paste_with_embedded_newline_produces_multiple_lines()
```

**Calls:** handle_event

### plain_at_sign_still_opens_the_file_picker

*Rust Function* — `src/tui/app.rs#L3187-L3194`

_private_

```
async fn plain_at_sign_still_opens_the_file_picker()
```

**Calls:** handle_chat_key, key

### plain_textarea

*Rust Function* — `src/tui/app.rs#L200-L204`

_private_

```
fn plain_textarea() -> TextArea<'static>
```

**Called by:** new, clear_input, set_input_text

### plan_task_error_marks_task_failed_and_stops_auto_execution

*Rust Function* — `src/tui/app.rs#L3811-L3853`

_private_

```
async fn plan_task_error_marks_task_failed_and_stops_auto_execution()
```

**Calls:** create_new_session, add_task, handle_event

### provider_switch_esc_returns_to_chat

*Rust Function* — `src/tui/app.rs#L3706-L3716`

_private_

```
async fn provider_switch_esc_returns_to_chat()
```

**Calls:** handle_provider_switch_key, key

### provider_switch_models_listed_clears_loading_state

*Rust Function* — `src/tui/app.rs#L3661-L3675`

_private_

```
async fn provider_switch_models_listed_clears_loading_state()
```

**Calls:** handle_event

### provider_switch_up_down_navigation_clamps_at_bounds

*Rust Function* — `src/tui/app.rs#L3678-L3703`

_private_

```
async fn provider_switch_up_down_navigation_clamps_at_bounds()
```

**Calls:** handle_provider_switch_key, key

### recalled_message_can_be_edited_before_resending

*Rust Function* — `src/tui/app.rs#L2948-L2958`

_private_

```
async fn recalled_message_can_be_edited_before_resending()
```

**Calls:** push_input_history, handle_chat_key, key

### send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight

*Rust Function* — `src/tui/app.rs#L4055-L4073`

_private_

```
async fn send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight()
```

**Calls:** create_new_session, len

### send_message_still_works_for_a_different_session_than_the_one_processing

*Rust Function* — `src/tui/app.rs#L4079-L4098`

_private_

```
async fn send_message_still_works_for_a_different_session_than_the_one_processing()
```

**Calls:** create_new_session, len

### setting_auto_mode_state_shares_the_same_cell_as_a_clone

*Rust Function* — `src/tui/app.rs#L3346-L3361`

_private_

```
async fn setting_auto_mode_state_shares_the_same_cell_as_a_clone()
```

**Calls:** set_auto_mode_state, handle_key_event, key

### shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps

*Rust Function* — `src/tui/app.rs#L3319-L3331`

_private_

```
async fn shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps()
```

**Calls:** handle_key_event, key

### shift_tab_works_from_any_mode_not_just_chat

*Rust Function* — `src/tui/app.rs#L3334-L3343`

_private_

```
async fn shift_tab_works_from_any_mode_not_just_chat()
```

**Calls:** handle_key_event, key

### skills_view_esc_returns_to_chat

*Rust Function* — `src/tui/app.rs#L3500-L3507`

_private_

```
async fn skills_view_esc_returns_to_chat()
```

**Calls:** handle_skills_key, key

### skills_view_up_down_navigation_clamps_at_bounds

*Rust Function* — `src/tui/app.rs#L3475-L3497`

_private_

```
async fn skills_view_up_down_navigation_clamps_at_bounds()
```

**Calls:** handle_skills_key, key

### slash_help_command_opens_help_view

*Rust Function* — `src/tui/app.rs#L3392-L3399`

_private_

```
async fn slash_help_command_opens_help_view()
```

### slash_mcp_command_opens_mcp_view

*Rust Function* — `src/tui/app.rs#L3375-L3389`

_private_

```
async fn slash_mcp_command_opens_mcp_view()
```

### slash_skills_command_opens_skills_view

*Rust Function* — `src/tui/app.rs#L3364-L3372`

_private_

```
async fn slash_skills_command_opens_skills_view()
```

### stale_session_response_chunk_is_dropped_after_switching_sessions

*Rust Function* — `src/tui/app.rs#L3767-L3803`

_private_

```
async fn stale_session_response_chunk_is_dropped_after_switching_sessions()
```

**Calls:** create_new_session, handle_event

### stale_session_response_complete_is_dropped_after_switching_sessions

*Rust Function* — `src/tui/app.rs#L3860-L3901`

_private_

```
async fn stale_session_response_complete_is_dropped_after_switching_sessions()
```

**Calls:** create_new_session, len, handle_event

### switch_provider_with_ollama_feature_swaps_provider_in_place

*Rust Function* — `src/tui/app.rs#L3738-L3756`

_private_

```
async fn switch_provider_with_ollama_feature_swaps_provider_in_place()
```

**Calls:** switch_provider_to_ollama_model

### switch_provider_without_ollama_feature_shows_clear_error

*Rust Function* — `src/tui/app.rs#L3720-L3734`

_private_

```
async fn switch_provider_without_ollama_feature_shows_clear_error()
```

**Calls:** switch_provider_to_ollama_model

### switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state

*Rust Function* — `src/tui/app.rs#L4025-L4047`

_private_

```
async fn switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state()
```

**Calls:** create_new_session, load_session

### switching_sessions_clears_a_stuck_processing_state_from_the_previous_session

*Rust Function* — `src/tui/app.rs#L3915-L3939`

_private_

```
async fn switching_sessions_clears_a_stuck_processing_state_from_the_previous_session()
```

**Calls:** create_new_session

### test_app

*Rust Function* — `src/tui/app.rs#L2559-L2565`

_private_

```
async fn test_app() -> App
```

**Calls:** run_migrations

### test_display_message_from_db_message

*Rust Function* — `src/tui/app.rs#L2505-L2522`

_private_

```
fn test_display_message_from_db_message()
```

### typed_backslashes_reach_the_input

*Rust Function* — `src/tui/app.rs#L3197-L3207`

_private_

```
async fn typed_backslashes_reach_the_input()
```

**Calls:** handle_chat_key, key

### typing_and_submitting_slash_skills_opens_the_dialog_end_to_end

*Rust Function* — `src/tui/app.rs#L3457-L3472`

_private_

```
async fn typing_and_submitting_slash_skills_opens_the_dialog_end_to_end()
```

**Calls:** handle_chat_key, key

### unrecognized_slash_word_falls_through_instead_of_being_swallowed

*Rust Function* — `src/tui/app.rs#L3402-L3414`

_private_

```
async fn unrecognized_slash_word_falls_through_instead_of_being_swallowed()
```

**Calls:** try_handle_slash_command

### up_is_plain_cursor_movement_when_there_is_no_history

*Rust Function* — `src/tui/app.rs#L2984-L2992`

_private_

```
async fn up_is_plain_cursor_movement_when_there_is_no_history()
```

**Calls:** set_input_text, handle_chat_key, key

### up_moves_the_cursor_inside_a_multiline_draft

*Rust Function* — `src/tui/app.rs#L2963-L2980`

_private_

```
async fn up_moves_the_cursor_inside_a_multiline_draft()
```

**Calls:** push_input_history, set_input_text, handle_chat_key, key

### up_recalls_previous_messages_without_sending_them

*Rust Function* — `src/tui/app.rs#L2879-L2907`

_private_

```
async fn up_recalls_previous_messages_without_sending_them()
```

**Calls:** push_input_history, set_input_text, handle_chat_key, key

### centered_rect

*Rust Function* — `src/tui/components/dialogs/mod.rs#L153-L171`

_private_

```
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect
```

**Called by:** render_crash_recovery_dialog, render_policy_denial

### render_auto_exec_progress

*Rust Function* — `src/tui/components/dialogs/mod.rs#L20-L71`

```
pub fn render_auto_exec_progress( f: &mut Frame, area: Rect, state: &PlanModeState, tasks: &[PlanTask], )
```

### render_crash_recovery_dialog

*Rust Function* — `src/tui/components/dialogs/mod.rs#L76-L122`

```
pub fn render_crash_recovery_dialog( f: &mut Frame, area: Rect, plan_title: &str, resume_at: usize, total: usize, )
```

**Calls:** centered_rect

### render_policy_denial

*Rust Function* — `src/tui/components/dialogs/mod.rs#L125-L150`

```
pub fn render_policy_denial(f: &mut Frame, area: Rect, tool_name: &str, reason: &str)
```

**Calls:** centered_rect

### get_croissant

*Rust Function* — `src/tui/components/logo.rs#L54-L56`

```
pub fn get_croissant() -> &'static str
```

### get_logo

*Rust Function* — `src/tui/components/logo.rs#L49-L51`

```
pub fn get_logo() -> &'static str
```

### get_logo_with_version

*Rust Function* — `src/tui/components/logo.rs#L64-L66`

```
pub fn get_logo_with_version(version: &str) -> String
```

**Called by:** test_logo_with_version

### get_small_logo

*Rust Function* — `src/tui/components/logo.rs#L59-L61`

```
pub fn get_small_logo() -> &'static str
```

### test_logo_not_empty

*Rust Function* — `src/tui/components/logo.rs#L73-L77`

_private_

```
fn test_logo_not_empty()
```

### test_logo_with_version

*Rust Function* — `src/tui/components/logo.rs#L80-L83`

_private_

```
fn test_logo_with_version()
```

**Calls:** get_logo_with_version

### name

*Rust Method* — `src/tui/error.rs#L72-L81`

```
pub fn name(&self) -> &'static str
```

### critical

*Rust Method* — `src/tui/error.rs#L144-L146`

```
pub fn critical(category: ErrorCategory, title: String, message: String) -> Self
```

### description

*Rust Method* — `src/tui/error.rs#L173-L215`

```
pub fn description(&self) -> Vec<String>
```

### error

*Rust Method* — `src/tui/error.rs#L139-L141`

```
pub fn error(category: ErrorCategory, title: String, message: String) -> Self
```

### from

*Rust Method* — `src/tui/error.rs#L225-L231`

_private_

```
fn from(message: &str) -> Self
```

### from

*Rust Method* — `src/tui/error.rs#L219-L221`

_private_

```
fn from(message: String) -> Self
```

### info

*Rust Method* — `src/tui/error.rs#L129-L131`

```
pub fn info(title: String, message: String) -> Self
```

### new

*Rust Method* — `src/tui/error.rs#L109-L126`

```
pub fn new( severity: ErrorSeverity, category: ErrorCategory, title: String, message: String, ) -> Self
```

### summary

*Rust Method* — `src/tui/error.rs#L163-L170`

```
pub fn summary(&self) -> String
```

**Called by:** test_error_info_summary

### warning

*Rust Method* — `src/tui/error.rs#L134-L136`

```
pub fn warning(category: ErrorCategory, title: String, message: String) -> Self
```

**Called by:** test_error_info_summary

### with_context

*Rust Method* — `src/tui/error.rs#L149-L152`

```
pub fn with_context(mut self, context: String) -> Self
```

**Called by:** cmd_keyring, cmd_ollama, merge_from_file, save, from_env, from_keyring, save_to_keyring, delete_from_keyring, connect, client_for, show_model, delete_model, pull_model, generate_embeddings, connect, send_request, read_response_line

### with_retry

*Rust Method* — `src/tui/error.rs#L155-L160`

```
pub fn with_retry(mut self, retry_count: u32, next_retry: DateTime<Utc>) -> Self
```

**Called by:** test_error_info_with_retry

### color

*Rust Method* — `src/tui/error.rs#L22-L30`

```
pub fn color(&self) -> ratatui::style::Color
```

### name

*Rust Method* — `src/tui/error.rs#L43-L50`

```
pub fn name(&self) -> &'static str
```

### prefix

*Rust Method* — `src/tui/error.rs#L33-L40`

```
pub fn prefix(&self) -> &'static str
```

### test_error_info_creation

*Rust Function* — `src/tui/error.rs#L249-L260`

_private_

```
fn test_error_info_creation()
```

### test_error_info_from_string

*Rust Function* — `src/tui/error.rs#L292-L296`

_private_

```
fn test_error_info_from_string()
```

### test_error_info_summary

*Rust Function* — `src/tui/error.rs#L278-L289`

_private_

```
fn test_error_info_summary()
```

**Calls:** warning, summary

### test_error_info_with_retry

*Rust Function* — `src/tui/error.rs#L263-L275`

_private_

```
fn test_error_info_with_retry()
```

**Calls:** with_retry

### test_error_severity_color

*Rust Function* — `src/tui/error.rs#L239-L246`

_private_

```
fn test_error_severity_color()
```

### default

*Rust Method* — `src/tui/events.rs#L258-L260`

_private_

```
fn default() -> Self
```

### new

*Rust Method* — `src/tui/events.rs#L192-L195`

```
pub fn new() -> Self
```

### next

*Rust Method* — `src/tui/events.rs#L211-L213`

```
pub async fn next(&mut self) -> Option<TuiEvent>
```

**Called by:** cmd_ollama, drain_stream_to_response, from_gemini_response, parse_gemini_sse, stream, stream, streamed_tool_call_reaches_caller, pull_model, from_openai_response, stream, from_qwen_response, stream, is_read_only_command, strip_verbatim_prefix, evaluate, find_active_shell_operator, append_fact, parse_skill_frontmatter_value, get_most_recent_plan, get_most_recent_session, next_event, try_handle_slash_command, test_streaming_basic, test_streaming_single_chunk, test_streaming_multiple_chunks, test_streaming_token_counting, test_streaming_stop_reason, test_streaming_error_handling, test_streaming_empty_response, test_streaming_content_accumulation

### sender

*Rust Method* — `src/tui/events.rs#L198-L200`

```
pub fn sender(&self) -> mpsc::UnboundedSender<TuiEvent>
```

**Called by:** event_sender, test_event_handler_creation

### start_terminal_listener

*Rust Method* — `src/tui/events.rs#L216-L254`

```
pub fn start_terminal_listener(tx: mpsc::UnboundedSender<TuiEvent>)
```

**Called by:** run_inner

### try_next

*Rust Method* — `src/tui/events.rs#L207-L209`

```
pub fn try_next(&mut self) -> Option<TuiEvent>
```

**Called by:** try_next_event

### is_timed_out

*Rust Method* — `src/tui/events.rs#L119-L121`

```
pub fn is_timed_out(&self) -> bool
```

**Called by:** handle_event

### time_remaining

*Rust Method* — `src/tui/events.rs#L124-L128`

```
pub fn time_remaining(&self) -> std::time::Duration
```

**Called by:** render_approval_header

### is_approve

*Rust Function* — `src/tui/events.rs#L390-L395`

```
pub fn is_approve(event: &KeyEvent) -> bool
```

**Calls:** is_empty

**Called by:** handle_approval_key

### is_cancel

*Rust Function* — `src/tui/events.rs#L360-L362`

```
pub fn is_cancel(event: &KeyEvent) -> bool
```

**Called by:** handle_key_event, handle_chat_key, handle_sessions_key, handle_skills_key, handle_mcp_key, handle_plan_key, handle_approval_key, handle_file_picker_key, handle_model_download_key, handle_provider_switch_key

### is_clear_session

*Rust Function* — `src/tui/events.rs#L293-L295`

```
pub fn is_clear_session(event: &KeyEvent) -> bool
```

**Calls:** key_matches

**Called by:** handle_key_event

### is_copy_response

*Rust Function* — `src/tui/events.rs#L319-L321`

```
pub fn is_copy_response(event: &KeyEvent) -> bool
```

**Calls:** key_matches

**Called by:** handle_chat_key

### is_deny

*Rust Function* — `src/tui/events.rs#L398-L403`

```
pub fn is_deny(event: &KeyEvent) -> bool
```

**Calls:** is_empty

**Called by:** handle_approval_key

### is_down

*Rust Function* — `src/tui/events.rs#L375-L377`

```
pub fn is_down(event: &KeyEvent) -> bool
```

**Calls:** is_empty

**Called by:** handle_sessions_key, handle_skills_key, handle_mcp_key, handle_file_picker_key, handle_model_download_key, handle_provider_switch_key

### is_enter

*Rust Function* — `src/tui/events.rs#L365-L367`

```
pub fn is_enter(event: &KeyEvent) -> bool
```

**Calls:** is_empty

**Called by:** handle_sessions_key, handle_file_picker_key, handle_model_download_key, handle_provider_switch_key

### is_help

*Rust Function* — `src/tui/events.rs#L288-L290`

```
pub fn is_help(event: &KeyEvent) -> bool
```

**Calls:** key_matches

**Called by:** handle_key_event

### is_list_sessions

*Rust Function* — `src/tui/events.rs#L283-L285`

```
pub fn is_list_sessions(event: &KeyEvent) -> bool
```

**Calls:** key_matches

**Called by:** handle_key_event

### is_model_download

*Rust Function* — `src/tui/events.rs#L303-L305`

```
pub fn is_model_download(event: &KeyEvent) -> bool
```

**Calls:** key_matches

**Called by:** handle_key_event

### is_model_info

*Rust Function* — `src/tui/events.rs#L308-L310`

```
pub fn is_model_info(event: &KeyEvent) -> bool
```

**Calls:** key_matches

**Called by:** handle_key_event

### is_new_session

*Rust Function* — `src/tui/events.rs#L278-L280`

```
pub fn is_new_session(event: &KeyEvent) -> bool
```

**Calls:** key_matches

**Called by:** handle_key_event

### is_newline

*Rust Function* — `src/tui/events.rs#L352-L357`

```
pub fn is_newline(event: &KeyEvent) -> bool
```

**Called by:** handle_chat_key

### is_page_down

*Rust Function* — `src/tui/events.rs#L385-L387`

```
pub fn is_page_down(event: &KeyEvent) -> bool
```

**Called by:** handle_chat_key

### is_page_up

*Rust Function* — `src/tui/events.rs#L380-L382`

```
pub fn is_page_up(event: &KeyEvent) -> bool
```

**Called by:** handle_chat_key

### is_paste_clipboard

*Rust Function* — `src/tui/events.rs#L326-L328`

```
pub fn is_paste_clipboard(event: &KeyEvent) -> bool
```

**Calls:** key_matches

**Called by:** handle_chat_key

### is_provider_switch

*Rust Function* — `src/tui/events.rs#L313-L315`

```
pub fn is_provider_switch(event: &KeyEvent) -> bool
```

**Calls:** key_matches

**Called by:** handle_key_event

### is_quit

*Rust Function* — `src/tui/events.rs#L273-L275`

```
pub fn is_quit(event: &KeyEvent) -> bool
```

**Calls:** key_matches

**Called by:** handle_key_event

### is_submit

*Rust Function* — `src/tui/events.rs#L341-L344`

```
pub fn is_submit(event: &KeyEvent) -> bool
```

**Calls:** is_empty

**Called by:** handle_chat_key

### is_toggle_auto_mode

*Rust Function* — `src/tui/events.rs#L334-L336`

```
pub fn is_toggle_auto_mode(event: &KeyEvent) -> bool
```

**Called by:** handle_key_event

### is_toggle_plan

*Rust Function* — `src/tui/events.rs#L298-L300`

```
pub fn is_toggle_plan(event: &KeyEvent) -> bool
```

**Calls:** key_matches

**Called by:** handle_key_event

### is_up

*Rust Function* — `src/tui/events.rs#L370-L372`

```
pub fn is_up(event: &KeyEvent) -> bool
```

**Calls:** is_empty

**Called by:** handle_sessions_key, handle_skills_key, handle_mcp_key, handle_file_picker_key, handle_model_download_key, handle_provider_switch_key

### is_view_details

*Rust Function* — `src/tui/events.rs#L406-L408`

```
pub fn is_view_details(event: &KeyEvent) -> bool
```

**Calls:** is_empty

**Called by:** handle_approval_key

### key_matches

*Rust Function* — `src/tui/events.rs#L264-L266`

```
pub fn key_matches(event: &KeyEvent, code: KeyCode, modifiers: KeyModifiers) -> bool
```

**Called by:** is_quit, is_new_session, is_list_sessions, is_help, is_clear_session, is_toggle_plan, is_model_download, is_model_info, is_provider_switch, is_copy_response, is_paste_clipboard

### test_copy_response_key

*Rust Function* — `src/tui/events.rs#L497-L503`

_private_

```
fn test_copy_response_key()
```

**Calls:** empty

### test_event_handler_creation

*Rust Function* — `src/tui/events.rs#L416-L421`

_private_

```
fn test_event_handler_creation()
```

**Calls:** sender

### test_key_matches

*Rust Function* — `src/tui/events.rs#L424-L436`

_private_

```
fn test_key_matches()
```

### test_model_info_key

*Rust Function* — `src/tui/events.rs#L479-L485`

_private_

```
fn test_model_info_key()
```

**Calls:** empty

### test_newline_key

*Rust Function* — `src/tui/events.rs#L524-L547`

_private_

```
fn test_newline_key()
```

**Calls:** empty

### test_paste_clipboard_key

*Rust Function* — `src/tui/events.rs#L506-L512`

_private_

```
fn test_paste_clipboard_key()
```

**Calls:** empty

### test_provider_switch_key

*Rust Function* — `src/tui/events.rs#L488-L494`

_private_

```
fn test_provider_switch_key()
```

**Calls:** empty

### test_quit_key

*Rust Function* — `src/tui/events.rs#L439-L445`

_private_

```
fn test_quit_key()
```

**Calls:** empty

### test_submit_key

*Rust Function* — `src/tui/events.rs#L448-L476`

_private_

```
fn test_submit_key()
```

**Calls:** empty

### test_toggle_auto_mode_key

*Rust Function* — `src/tui/events.rs#L515-L521`

_private_

```
fn test_toggle_auto_mode_key()
```

**Calls:** empty

### find_syntax

*Rust Function* — `src/tui/highlight.rs#L51-L65`

_private_

```
fn find_syntax(language: &str) -> Option<&'static SyntaxReference>
```

**Called by:** highlight_code, is_language_supported

### get_theme

*Rust Function* — `src/tui/highlight.rs#L24-L26`

_private_

```
fn get_theme() -> &'static Theme
```

**Called by:** highlight_code

### highlight_code

*Rust Function* — `src/tui/highlight.rs#L70-L133`

```
pub fn highlight_code(code: &str, language: &str) -> Vec<Line<'static>>
```

**Calls:** find_syntax, get_theme, syntect_style_to_ratatui

**Called by:** test_highlight_rust, test_highlight_python, test_highlight_javascript, test_highlight_unknown_language, test_empty_code, test_code_with_special_characters, end_code_block

### is_language_supported

*Rust Function* — `src/tui/highlight.rs#L145-L147`

```
pub fn is_language_supported(language: &str) -> bool
```

**Calls:** find_syntax

### supported_languages

*Rust Function* — `src/tui/highlight.rs#L136-L142`

```
pub fn supported_languages() -> Vec<String>
```

**Called by:** test_supported_languages

### syntect_style_to_ratatui

*Rust Function* — `src/tui/highlight.rs#L34-L48`

_private_

```
fn syntect_style_to_ratatui(syntect_style: syntect::highlighting::Style) -> Style
```

**Calls:** syntect_to_ratatui_color

**Called by:** highlight_code

### syntect_to_ratatui_color

*Rust Function* — `src/tui/highlight.rs#L29-L31`

_private_

```
fn syntect_to_ratatui_color(color: syntect::highlighting::Color) -> Color
```

**Called by:** syntect_style_to_ratatui

### test_code_with_special_characters

*Rust Function* — `src/tui/highlight.rs#L209-L213`

_private_

```
fn test_code_with_special_characters()
```

**Calls:** highlight_code

### test_empty_code

*Rust Function* — `src/tui/highlight.rs#L201-L206`

_private_

```
fn test_empty_code()
```

**Calls:** highlight_code

### test_highlight_javascript

*Rust Function* — `src/tui/highlight.rs#L169-L173`

_private_

```
fn test_highlight_javascript()
```

**Calls:** highlight_code

### test_highlight_python

*Rust Function* — `src/tui/highlight.rs#L162-L166`

_private_

```
fn test_highlight_python()
```

**Calls:** highlight_code

### test_highlight_rust

*Rust Function* — `src/tui/highlight.rs#L154-L159`

_private_

```
fn test_highlight_rust()
```

**Calls:** highlight_code

### test_highlight_unknown_language

*Rust Function* — `src/tui/highlight.rs#L176-L181`

_private_

```
fn test_highlight_unknown_language()
```

**Calls:** highlight_code

### test_is_language_supported

*Rust Function* — `src/tui/highlight.rs#L192-L198`

_private_

```
fn test_is_language_supported()
```

### test_supported_languages

*Rust Function* — `src/tui/highlight.rs#L184-L189`

_private_

```
fn test_supported_languages()
```

**Calls:** supported_languages

### end_code_block

*Rust Method* — `src/tui/markdown.rs#L122-L145`

_private_

```
fn end_code_block(&mut self)
```

**Calls:** flush_current_line, is_empty, highlight_code

**Called by:** handle_end_tag

### end_heading

*Rust Method* — `src/tui/markdown.rs#L93-L120`

_private_

```
fn end_heading(&mut self)
```

**Calls:** is_empty

**Called by:** handle_end_tag

### end_list

*Rust Method* — `src/tui/markdown.rs#L147-L152`

_private_

```
fn end_list(&mut self)
```

**Called by:** handle_end_tag

### end_paragraph

*Rust Method* — `src/tui/markdown.rs#L154-L157`

_private_

```
fn end_paragraph(&mut self)
```

**Calls:** flush_current_line

**Called by:** handle_end_tag

### finish

*Rust Method* — `src/tui/markdown.rs#L204-L212`

_private_

```
fn finish(mut self) -> Vec<Line<'static>>
```

**Calls:** is_empty

**Called by:** bench_session_create, bench_session_get, bench_session_list, bench_message_insert, bench_message_query, bench_parallel_dispatch, from_tool, parse_markdown

### flush_current_line

*Rust Method* — `src/tui/markdown.rs#L54-L59`

_private_

```
fn flush_current_line(&mut self)
```

**Calls:** is_empty

**Called by:** start_code_block, handle_start_tag, end_code_block, end_paragraph, handle_end_tag, handle_rule, parse_markdown

### handle_end_tag

*Rust Method* — `src/tui/markdown.rs#L159-L169`

_private_

```
fn handle_end_tag(&mut self, tag: TagEnd)
```

**Calls:** end_heading, end_code_block, end_list, end_paragraph, flush_current_line

**Called by:** parse_markdown

### handle_inline_code

*Rust Method* — `src/tui/markdown.rs#L183-L191`

_private_

```
fn handle_inline_code(&mut self, code: pulldown_cmark::CowStr<'_>)
```

**Called by:** parse_markdown

### handle_rule

*Rust Method* — `src/tui/markdown.rs#L193-L200`

_private_

```
fn handle_rule(&mut self)
```

**Calls:** flush_current_line

**Called by:** parse_markdown

### handle_start_tag

*Rust Method* — `src/tui/markdown.rs#L83-L91`

_private_

```
fn handle_start_tag(&mut self, tag: Tag)
```

**Calls:** start_code_block, flush_current_line

**Called by:** parse_markdown

### handle_text

*Rust Method* — `src/tui/markdown.rs#L171-L181`

_private_

```
fn handle_text(&mut self, text: pulldown_cmark::CowStr<'_>)
```

**Called by:** parse_markdown

### new

*Rust Method* — `src/tui/markdown.rs#L42-L52`

_private_

```
fn new() -> Self
```

### start_code_block

*Rust Method* — `src/tui/markdown.rs#L61-L81`

_private_

```
fn start_code_block(&mut self, kind: CodeBlockKind)
```

**Calls:** is_empty, flush_current_line

**Called by:** handle_start_tag

### last_code_block

*Rust Function* — `src/tui/markdown.rs#L238-L262`

```
pub fn last_code_block(markdown: &str) -> Option<String>
```

**Called by:** copy_last_response_to_clipboard

### last_code_block_extracts_fenced_content

*Rust Function* — `src/tui/markdown.rs#L353-L360`

_private_

```
fn last_code_block_extracts_fenced_content()
```

### last_code_block_returns_none_without_any_code

*Rust Function* — `src/tui/markdown.rs#L369-L372`

_private_

```
fn last_code_block_returns_none_without_any_code()
```

### last_code_block_returns_the_last_of_multiple_blocks

*Rust Function* — `src/tui/markdown.rs#L363-L366`

_private_

```
fn last_code_block_returns_the_last_of_multiple_blocks()
```

### markdown_escapes_backslash_before_punctuation

*Rust Function* — `src/tui/markdown.rs#L322-L329`

_private_

```
fn markdown_escapes_backslash_before_punctuation()
```

**Calls:** rendered_text, parse_markdown

### parse_markdown

*Rust Function* — `src/tui/markdown.rs#L216-L232`

```
pub fn parse_markdown(markdown: &str) -> Vec<Line<'static>>
```

**Calls:** handle_start_tag, handle_end_tag, handle_text, handle_inline_code, flush_current_line, handle_rule, finish

**Called by:** markdown_escapes_backslash_before_punctuation, test_parse_simple_text, test_parse_heading, test_parse_code_block, test_parse_inline_code, test_parse_list, test_parse_horizontal_rule, test_empty_markdown, render_message_lines, render_streaming_response

### parse_plain_text

*Rust Function* — `src/tui/markdown.rs#L19-L23`

```
pub fn parse_plain_text(text: &str) -> Vec<Line<'static>>
```

**Called by:** plain_text_keeps_windows_path_backslashes, plain_text_keeps_markdown_syntax_literal, plain_text_preserves_line_structure, render_message_lines

### plain_text_keeps_markdown_syntax_literal

*Rust Function* — `src/tui/markdown.rs#L307-L311`

_private_

```
fn plain_text_keeps_markdown_syntax_literal()
```

**Calls:** rendered_text, parse_plain_text

### plain_text_keeps_windows_path_backslashes

*Rust Function* — `src/tui/markdown.rs#L293-L304`

_private_

```
fn plain_text_keeps_windows_path_backslashes()
```

**Calls:** rendered_text, parse_plain_text

### plain_text_preserves_line_structure

*Rust Function* — `src/tui/markdown.rs#L314-L317`

_private_

```
fn plain_text_preserves_line_structure()
```

**Calls:** rendered_text, parse_plain_text

### rendered_text

*Rust Function* — `src/tui/markdown.rs#L270-L281`

_private_

```
fn rendered_text(lines: &[Line<'static>]) -> String
```

**Called by:** plain_text_keeps_windows_path_backslashes, plain_text_keeps_markdown_syntax_literal, plain_text_preserves_line_structure, markdown_escapes_backslash_before_punctuation

### test_empty_markdown

*Rust Function* — `src/tui/markdown.rs#L396-L400`

_private_

```
fn test_empty_markdown()
```

**Calls:** parse_markdown

### test_parse_code_block

*Rust Function* — `src/tui/markdown.rs#L346-L350`

_private_

```
fn test_parse_code_block()
```

**Calls:** parse_markdown

### test_parse_heading

*Rust Function* — `src/tui/markdown.rs#L339-L343`

_private_

```
fn test_parse_heading()
```

**Calls:** parse_markdown

### test_parse_horizontal_rule

*Rust Function* — `src/tui/markdown.rs#L389-L393`

_private_

```
fn test_parse_horizontal_rule()
```

**Calls:** parse_markdown

### test_parse_inline_code

*Rust Function* — `src/tui/markdown.rs#L375-L379`

_private_

```
fn test_parse_inline_code()
```

**Calls:** parse_markdown

### test_parse_list

*Rust Function* — `src/tui/markdown.rs#L382-L386`

_private_

```
fn test_parse_list()
```

**Calls:** parse_markdown

### test_parse_simple_text

*Rust Function* — `src/tui/markdown.rs#L332-L336`

_private_

```
fn test_parse_simple_text()
```

**Calls:** parse_markdown

### fraction

*Rust Method* — `src/tui/ollama_download.rs#L42-L46`

```
pub fn fraction(&self) -> Option<f64>
```

### build_ollama_provider

*Rust Function* — `src/tui/ollama_download.rs#L104-L123`

```
pub fn build_ollama_provider( host: &str, model: &str, config: Option<&crate::config::OllamaProviderConfig>, ) -> Result<std::sync::Arc<dyn crate::llm::provider::Provider>, String>
```

**Calls:** ollama_provider_from_config

### build_ollama_provider

*Rust Function* — `src/tui/ollama_download.rs#L126-L136`

```
pub fn build_ollama_provider( _host: &str, _model: &str, _config: Option<&crate::config::OllamaProviderConfig>, ) -> Result<std::sync::Arc<dyn crate::llm::provider::Provider>, String>
```

### fetch_installed_models

*Rust Function* — `src/tui/ollama_download.rs#L79-L84`

```
pub async fn fetch_installed_models(host: String) -> Vec<String>
```

**Calls:** list_models

### fetch_installed_models

*Rust Function* — `src/tui/ollama_download.rs#L87-L89`

```
pub async fn fetch_installed_models(_host: String) -> Vec<String>
```

### filter_suggestions

*Rust Function* — `src/tui/ollama_download.rs#L53-L73`

```
pub fn filter_suggestions(query: &str, installed: &[String]) -> Vec<String>
```

**Calls:** is_empty

**Called by:** refresh_model_download_suggestions, filter_suggestions_empty_query_returns_all_deduped, filter_suggestions_matches_substring_case_insensitive, filter_suggestions_includes_ornith

### filter_suggestions_empty_query_returns_all_deduped

*Rust Function* — `src/tui/ollama_download.rs#L266-L279`

_private_

```
fn filter_suggestions_empty_query_returns_all_deduped()
```

**Calls:** filter_suggestions

### filter_suggestions_includes_ornith

*Rust Function* — `src/tui/ollama_download.rs#L290-L293`

_private_

```
fn filter_suggestions_includes_ornith()
```

**Calls:** filter_suggestions

### filter_suggestions_matches_substring_case_insensitive

*Rust Function* — `src/tui/ollama_download.rs#L282-L287`

_private_

```
fn filter_suggestions_matches_substring_case_insensitive()
```

**Calls:** filter_suggestions

### pull_progress_fraction

*Rust Function* — `src/tui/ollama_download.rs#L296-L303`

_private_

```
fn pull_progress_fraction()
```

### spawn_delete

*Rust Function* — `src/tui/ollama_download.rs#L196-L210`

```
pub async fn spawn_delete( host: String, model: String, event_sender: UnboundedSender<TuiEvent>, ) -> JoinHandle<()>
```

**Calls:** delete_model

### spawn_delete

*Rust Function* — `src/tui/ollama_download.rs#L213-L228`

```
pub async fn spawn_delete( _host: String, model: String, event_sender: UnboundedSender<TuiEvent>, ) -> JoinHandle<()>
```

### spawn_pull

*Rust Function* — `src/tui/ollama_download.rs#L142-L171`

```
pub async fn spawn_pull( host: String, model: String, event_sender: UnboundedSender<TuiEvent>, ) -> JoinHandle<()>
```

**Calls:** pull_model

### spawn_pull

*Rust Function* — `src/tui/ollama_download.rs#L174-L189`

```
pub async fn spawn_pull( _host: String, model: String, event_sender: UnboundedSender<TuiEvent>, ) -> JoinHandle<()>
```

### switch_built_provider_applies_per_model_num_ctx_from_config

*Rust Function* — `src/tui/ollama_download.rs#L242-L263`

_private_

```
fn switch_built_provider_applies_per_model_num_ctx_from_config()
```

### analyze_and_transform

*Rust Method* — `src/tui/prompt_analyzer.rs#L127-L197`

```
pub fn analyze_and_transform(&self, prompt: &str) -> String
```

**Calls:** is_empty

**Called by:** send_message, test_plan_detection, test_read_file_detection, test_search_detection, test_multiple_detections, test_no_detection, test_case_insensitive, test_web_search_detection, test_bash_detection

### build_keyword_regex

*Rust Method* — `src/tui/prompt_analyzer.rs#L117-L124`

_private_

```
fn build_keyword_regex(keywords: &[&str]) -> Regex
```

**Called by:** new

### classify_tier

*Rust Method* — `src/tui/prompt_analyzer.rs#L252-L278`

```
pub fn classify_tier(&self, prompt: &str) -> crate::llm::provider::router::ModelTier
```

**Called by:** send_message_with_tools_inner, complex_prompt_routes_to_powerful_tier, simple_prompt_routes_to_fast_tier, neutral_prompt_routes_to_balanced_tier

### default

*Rust Method* — `src/tui/prompt_analyzer.rs#L201-L203`

_private_

```
fn default() -> Self
```

### new

*Rust Method* — `src/tui/prompt_analyzer.rs#L104-L114`

```
pub fn new() -> Self
```

**Calls:** build_keyword_regex

### test_bash_detection

*Rust Function* — `src/tui/prompt_analyzer.rs#L355-L362`

_private_

```
fn test_bash_detection()
```

**Calls:** analyze_and_transform

### test_case_insensitive

*Rust Function* — `src/tui/prompt_analyzer.rs#L335-L342`

_private_

```
fn test_case_insensitive()
```

**Calls:** analyze_and_transform

### test_multiple_detections

*Rust Function* — `src/tui/prompt_analyzer.rs#L316-L323`

_private_

```
fn test_multiple_detections()
```

**Calls:** analyze_and_transform

### test_no_detection

*Rust Function* — `src/tui/prompt_analyzer.rs#L326-L332`

_private_

```
fn test_no_detection()
```

**Calls:** analyze_and_transform

### test_plan_detection

*Rust Function* — `src/tui/prompt_analyzer.rs#L286-L293`

_private_

```
fn test_plan_detection()
```

**Calls:** analyze_and_transform

### test_read_file_detection

*Rust Function* — `src/tui/prompt_analyzer.rs#L296-L303`

_private_

```
fn test_read_file_detection()
```

**Calls:** analyze_and_transform

### test_search_detection

*Rust Function* — `src/tui/prompt_analyzer.rs#L306-L313`

_private_

```
fn test_search_detection()
```

**Calls:** analyze_and_transform

### test_web_search_detection

*Rust Function* — `src/tui/prompt_analyzer.rs#L345-L352`

_private_

```
fn test_web_search_detection()
```

**Calls:** analyze_and_transform

### calculate_cost

*Rust Method* — `src/tui/render.rs#L2070-L2072`

_private_

```
fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64
```

### complete

*Rust Method* — `src/tui/render.rs#L2052-L2054`

_private_

```
async fn complete(&self, _request: LLMRequest) -> ProviderResult<LLMResponse>
```

### context_window

*Rust Method* — `src/tui/render.rs#L2067-L2069`

_private_

```
fn context_window(&self, _model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/tui/render.rs#L2061-L2063`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/tui/render.rs#L2058-L2060`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/tui/render.rs#L2055-L2057`

_private_

```
async fn stream(&self, _request: LLMRequest) -> ProviderResult<ProviderStream>
```

### supported_models

*Rust Method* — `src/tui/render.rs#L2064-L2066`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### approval_dialog_area

*Rust Function* — `src/tui/render.rs#L1204-L1224`

_private_

```
fn approval_dialog_area(area: Rect, show_details: bool) -> Rect
```

**Called by:** render_approval

### approval_dialog_details_view_shows_pretty_printed_json

*Rust Function* — `src/tui/render.rs#L2616-L2628`

_private_

```
async fn approval_dialog_details_view_shows_pretty_printed_json()
```

**Calls:** test_approval_request, render_to_string

### approval_dialog_shows_tool_name_capabilities_and_summarized_params

*Rust Function* — `src/tui/render.rs#L2597-L2613`

_private_

```
async fn approval_dialog_shows_tool_name_capabilities_and_summarized_params()
```

**Calls:** test_approval_request, render_to_string

### chat_input_renders_textarea_contents_and_hint

*Rust Function* — `src/tui/render.rs#L2347-L2355`

_private_

```
async fn chat_input_renders_textarea_contents_and_hint()
```

**Calls:** render_to_string

### chat_message_perf_footer_reports_cold_and_warm_starts

*Rust Function* — `src/tui/render.rs#L2489-L2518`

_private_

```
async fn chat_message_perf_footer_reports_cold_and_warm_starts()
```

**Calls:** render_to_string

### chat_message_thinking_block_toggles_between_collapsed_and_expanded

*Rust Function* — `src/tui/render.rs#L2460-L2486`

_private_

```
async fn chat_message_thinking_block_toggles_between_collapsed_and_expanded()
```

**Calls:** render_to_string

### chat_shows_pending_plan_banner_only_while_awaiting_approval

*Rust Function* — `src/tui/render.rs#L2440-L2457`

_private_

```
async fn chat_shows_pending_plan_banner_only_while_awaiting_approval()
```

**Calls:** render_to_string

### chat_shows_streaming_response_and_processing_indicator

*Rust Function* — `src/tui/render.rs#L2521-L2535`

_private_

```
async fn chat_shows_streaming_response_and_processing_indicator()
```

**Calls:** render_to_string

### compute_scroll_offset

*Rust Function* — `src/tui/render.rs#L425-L428`

_private_

```
fn compute_scroll_offset(total_lines: usize, visible_height: usize, scroll_offset: usize) -> u16
```

**Called by:** render_chat

### feature_row

*Rust Function* — `src/tui/render.rs#L712-L723`

_private_

```
fn feature_row(name: &'static str, desc: &'static str) -> Line<'static>
```

**Called by:** help_features

### header_omits_tokens_per_second_when_unavailable

*Rust Function* — `src/tui/render.rs#L2194-L2201`

_private_

```
async fn header_omits_tokens_per_second_when_unavailable()
```

**Calls:** render_to_string

### header_shows_ollama_provider_badge_and_tokens_per_second

*Rust Function* — `src/tui/render.rs#L2157-L2191`

_private_

```
async fn header_shows_ollama_provider_badge_and_tokens_per_second()
```

**Calls:** render_to_string

### help_chat_mode

*Rust Function* — `src/tui/render.rs#L771-L824`

_private_

```
fn help_chat_mode(app: &App) -> Vec<Line<'static>>
```

**Calls:** help_section_header, help_row

**Called by:** render_help

### help_features

*Rust Function* — `src/tui/render.rs#L864-L884`

_private_

```
fn help_features() -> Vec<Line<'static>>
```

**Calls:** help_section_header, feature_row

**Called by:** render_help

### help_footer

*Rust Function* — `src/tui/render.rs#L886-L904`

_private_

```
fn help_footer() -> Vec<Line<'static>>
```

**Called by:** render_help

### help_global_commands

*Rust Function* — `src/tui/render.rs#L734-L769`

_private_

```
fn help_global_commands() -> Vec<Line<'static>>
```

**Calls:** help_section_header, help_row

**Called by:** render_help

### help_plan_mode

*Rust Function* — `src/tui/render.rs#L842-L862`

_private_

```
fn help_plan_mode() -> Vec<Line<'static>>
```

**Calls:** help_section_header, help_row

**Called by:** render_help

### help_row

*Rust Function* — `src/tui/render.rs#L700-L709`

_private_

```
fn help_row(key: &'static str, desc: impl Into<String>, key_color: Color) -> Line<'static>
```

**Called by:** help_global_commands, help_chat_mode, help_session_list, help_plan_mode

### help_screen_lists_commands_from_every_section

*Rust Function* — `src/tui/render.rs#L2408-L2427`

_private_

```
async fn help_screen_lists_commands_from_every_section()
```

**Calls:** render_to_string

### help_screen_shows_shift_enter_when_kitty_protocol_active

*Rust Function* — `src/tui/render.rs#L2430-L2437`

_private_

```
async fn help_screen_shows_shift_enter_when_kitty_protocol_active()
```

**Calls:** render_to_string

### help_section_header

*Rust Function* — `src/tui/render.rs#L726-L732`

_private_

```
fn help_section_header(title: &'static str) -> [Line<'static>; 3]
```

**Called by:** help_global_commands, help_chat_mode, help_session_list, help_plan_mode, help_features

### help_session_list

*Rust Function* — `src/tui/render.rs#L826-L840`

_private_

```
fn help_session_list() -> Vec<Line<'static>>
```

**Calls:** help_section_header, help_row

**Called by:** render_help

### line_text

*Rust Function* — `src/tui/render.rs#L2102-L2104`

_private_

```
fn line_text(line: &Line<'_>) -> String
```

**Called by:** message_header_timestamp_is_shown_in_local_time

### mcp_view_shows_connected_server_with_tool_count

*Rust Function* — `src/tui/render.rs#L2256-L2270`

_private_

```
async fn mcp_view_shows_connected_server_with_tool_count()
```

**Calls:** render_to_string

### mcp_view_shows_connection_error

*Rust Function* — `src/tui/render.rs#L2273-L2288`

_private_

```
async fn mcp_view_shows_connection_error()
```

**Calls:** render_to_string

### mcp_view_shows_empty_state_message

*Rust Function* — `src/tui/render.rs#L2291-L2297`

_private_

```
async fn mcp_view_shows_empty_state_message()
```

**Calls:** render_to_string

### message_header_timestamp_is_shown_in_local_time

*Rust Function* — `src/tui/render.rs#L2111-L2154`

_private_

```
fn message_header_timestamp_is_shown_in_local_time()
```

**Calls:** line_text, render_message_lines

### model_download_confirm_delete_shows_prompt

*Rust Function* — `src/tui/render.rs#L2326-L2334`

_private_

```
async fn model_download_confirm_delete_shows_prompt()
```

**Calls:** render_to_string

### model_download_deleting_shows_status

*Rust Function* — `src/tui/render.rs#L2337-L2344`

_private_

```
async fn model_download_deleting_shows_status()
```

**Calls:** render_to_string

### model_download_dialog_shows_prompt_and_suggestions

*Rust Function* — `src/tui/render.rs#L2300-L2308`

_private_

```
async fn model_download_dialog_shows_prompt_and_suggestions()
```

**Calls:** render_to_string

### model_download_progress_shows_status_and_bar

*Rust Function* — `src/tui/render.rs#L2311-L2323`

_private_

```
async fn model_download_progress_shows_status_and_bar()
```

**Calls:** render_to_string

### model_info_panel_shows_last_response_perf_metrics

*Rust Function* — `src/tui/render.rs#L2370-L2405`

_private_

```
async fn model_info_panel_shows_last_response_perf_metrics()
```

**Calls:** render_to_string

### model_info_panel_shows_provider_model_and_context_window

*Rust Function* — `src/tui/render.rs#L2358-L2367`

_private_

```
async fn model_info_panel_shows_provider_model_and_context_window()
```

**Calls:** render_to_string

### plan_mode_shows_empty_state_without_a_plan

*Rust Function* — `src/tui/render.rs#L2572-L2578`

_private_

```
async fn plan_mode_shows_empty_state_without_a_plan()
```

**Calls:** render_to_string

### plan_mode_shows_full_document_with_tasks_and_criteria

*Rust Function* — `src/tui/render.rs#L2538-L2569`

_private_

```
async fn plan_mode_shows_full_document_with_tasks_and_criteria()
```

**Calls:** render_to_string

### provider_icon

*Rust Function* — `src/tui/render.rs#L89-L98`

_private_

```
fn provider_icon(provider_name: &str) -> &'static str
```

### render

*Rust Function* — `src/tui/render.rs#L19-L85`

```
pub fn render(f: &mut Frame, app: &App)
```

**Calls:** render_splash, render_header, render_chat, render_input, render_plan, render_plan_help, render_sessions, render_help, render_settings, render_approval, render_file_picker, render_model_download, render_model_info, render_provider_switch, render_skills, render_mcp, render_status_bar

**Called by:** render_to_string, run_loop

### render_approval

*Rust Function* — `src/tui/render.rs#L1398-L1433`

_private_

```
fn render_approval(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** approval_dialog_area, render_approval_header, render_approval_capabilities, render_approval_input_detailed, render_approval_input_summary, render_approval_actions, block

**Called by:** render

### render_approval_actions

*Rust Function* — `src/tui/render.rs#L1367-L1396`

_private_

```
fn render_approval_actions() -> Line<'static>
```

**Called by:** render_approval

### render_approval_capabilities

*Rust Function* — `src/tui/render.rs#L1282-L1299`

_private_

```
fn render_approval_capabilities(request: &super::events::ToolApprovalRequest) -> Vec<Line<'_>>
```

**Calls:** is_empty

**Called by:** render_approval

### render_approval_header

*Rust Function* — `src/tui/render.rs#L1227-L1279`

_private_

```
fn render_approval_header<'a>( request: &'a super::events::ToolApprovalRequest, model_name: &'a str, ) -> Vec<Line<'a>>
```

**Calls:** time_remaining

**Called by:** render_approval

### render_approval_input_detailed

*Rust Function* — `src/tui/render.rs#L1303-L1321`

_private_

```
fn render_approval_input_detailed(request: &super::events::ToolApprovalRequest) -> Vec<Line<'_>>
```

**Called by:** render_approval

### render_approval_input_summary

*Rust Function* — `src/tui/render.rs#L1325-L1364`

_private_

```
fn render_approval_input_summary(request: &super::events::ToolApprovalRequest) -> Vec<Line<'_>>
```

**Calls:** is_empty, len

**Called by:** render_approval

### render_chat

*Rust Function* — `src/tui/render.rs#L431-L471`

_private_

```
fn render_chat(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** render_pending_plan_banner, render_message_lines, render_streaming_response, render_processing_indicator, len, compute_scroll_offset, block

**Called by:** render

### render_file_picker

*Rust Function* — `src/tui/render.rs#L1436-L1553`

_private_

```
fn render_file_picker(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** len, skip, block

**Called by:** render

### render_header

*Rust Function* — `src/tui/render.rs#L101-L197`

_private_

```
fn render_header(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** total_tokens, total_cost, len, block

**Called by:** render

### render_help

*Rust Function* — `src/tui/render.rs#L907-L942`

_private_

```
fn render_help(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** help_global_commands, help_chat_mode, help_session_list, help_plan_mode, help_features, help_footer, block

**Called by:** render

### render_input

*Rust Function* — `src/tui/render.rs#L478-L521`

_private_

```
fn render_input(f: &mut Frame, app: &App, area: Rect)
```

**Called by:** render

### render_mcp

*Rust Function* — `src/tui/render.rs#L636-L697`

_private_

```
fn render_mcp(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** is_empty, block

**Called by:** render

### render_message_lines

*Rust Function* — `src/tui/render.rs#L319-L374`

_private_

```
fn render_message_lines(msg: &super::app::DisplayMessage, model_name: &str) -> Vec<Line<'static>>
```

**Calls:** render_thinking_block, parse_plain_text, parse_markdown, render_perf_footer

**Called by:** render_chat, message_header_timestamp_is_shown_in_local_time

### render_model_download

*Rust Function* — `src/tui/render.rs#L1726-L1848`

_private_

```
fn render_model_download(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** render_model_download_confirm_delete, render_model_download_deleting, render_model_download_progress, is_empty, block

**Called by:** render

### render_model_download_confirm_delete

*Rust Function* — `src/tui/render.rs#L1908-L1949`

_private_

```
fn render_model_download_confirm_delete(f: &mut Frame, model: &str, area: Rect)
```

**Calls:** block

**Called by:** render_model_download

### render_model_download_deleting

*Rust Function* — `src/tui/render.rs#L1952-L1975`

_private_

```
fn render_model_download_deleting(f: &mut Frame, model: &str, area: Rect)
```

**Calls:** block

**Called by:** render_model_download

### render_model_download_progress

*Rust Function* — `src/tui/render.rs#L1851-L1904`

_private_

```
fn render_model_download_progress(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** block

**Called by:** render_model_download

### render_model_info

*Rust Function* — `src/tui/render.rs#L1557-L1666`

_private_

```
fn render_model_info(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** last_assistant_message, block

**Called by:** render

### render_pending_plan_banner

*Rust Function* — `src/tui/render.rs#L200-L239`

_private_

```
fn render_pending_plan_banner(app: &App) -> Vec<Line<'static>>
```

**Called by:** render_chat

### render_perf_footer

*Rust Function* — `src/tui/render.rs#L243-L276`

_private_

```
fn render_perf_footer(msg: &super::app::DisplayMessage) -> Option<Line<'static>>
```

**Called by:** render_message_lines

### render_plan

*Rust Function* — `src/tui/render.rs#L1152-L1174`

_private_

```
fn render_plan(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** render_plan_document, block, render_plan_empty_state

**Called by:** render

### render_plan_document

*Rust Function* — `src/tui/render.rs#L1042-L1132`

_private_

```
fn render_plan_document(plan: &crate::plan::PlanDocument, area_width: usize) -> Vec<Line<'_>>
```

**Calls:** is_empty, render_plan_task_lines

**Called by:** render_plan

### render_plan_empty_state

*Rust Function* — `src/tui/render.rs#L1135-L1150`

_private_

```
fn render_plan_empty_state() -> Vec<Line<'static>>
```

**Called by:** render_plan

### render_plan_help

*Rust Function* — `src/tui/render.rs#L945-L997`

_private_

```
fn render_plan_help(f: &mut Frame, area: Rect)
```

**Calls:** block

**Called by:** render

### render_plan_task_lines

*Rust Function* — `src/tui/render.rs#L1003-L1038`

_private_

```
fn render_plan_task_lines(task: &crate::plan::PlanTask, idx: usize) -> Vec<Line<'_>>
```

**Calls:** is_empty

**Called by:** render_plan_document

### render_processing_indicator

*Rust Function* — `src/tui/render.rs#L397-L420`

_private_

```
fn render_processing_indicator(app: &App, model_name: &str) -> Vec<Line<'static>>
```

**Calls:** len

**Called by:** render_chat

### render_provider_switch

*Rust Function* — `src/tui/render.rs#L1670-L1721`

_private_

```
fn render_provider_switch(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** is_empty, block

**Called by:** render

### render_sessions

*Rust Function* — `src/tui/render.rs#L524-L574`

_private_

```
fn render_sessions(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** block

**Called by:** render

### render_settings

*Rust Function* — `src/tui/render.rs#L1177-L1199`

_private_

```
fn render_settings(f: &mut Frame, _app: &App, area: Rect)
```

**Calls:** block

**Called by:** render

### render_skills

*Rust Function* — `src/tui/render.rs#L577-L633`

_private_

```
fn render_skills(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** is_empty, block

**Called by:** render

### render_status_bar

*Rust Function* — `src/tui/render.rs#L1978-L2029`

_private_

```
fn render_status_bar(f: &mut Frame, app: &App, area: Rect)
```

**Calls:** auto_mode

**Called by:** render

### render_streaming_response

*Rust Function* — `src/tui/render.rs#L377-L393`

_private_

```
fn render_streaming_response(app: &App, model_name: &str) -> Vec<Line<'static>>
```

**Calls:** parse_markdown

**Called by:** render_chat

### render_thinking_block

*Rust Function* — `src/tui/render.rs#L279-L315`

_private_

```
fn render_thinking_block(msg: &super::app::DisplayMessage) -> Vec<Line<'static>>
```

**Called by:** render_message_lines

### render_to_string

*Rust Function* — `src/tui/render.rs#L2085-L2099`

_private_

```
fn render_to_string(app: &App, width: u16, height: u16) -> String
```

**Calls:** render

**Called by:** header_shows_ollama_provider_badge_and_tokens_per_second, header_omits_tokens_per_second_when_unavailable, status_bar_shows_interactive_by_default, status_bar_shows_full_auto_when_active, skills_view_shows_name_and_description, skills_view_shows_empty_state_message, mcp_view_shows_connected_server_with_tool_count, mcp_view_shows_connection_error, mcp_view_shows_empty_state_message, model_download_dialog_shows_prompt_and_suggestions, model_download_progress_shows_status_and_bar, model_download_confirm_delete_shows_prompt, model_download_deleting_shows_status, chat_input_renders_textarea_contents_and_hint, model_info_panel_shows_provider_model_and_context_window, model_info_panel_shows_last_response_perf_metrics, help_screen_lists_commands_from_every_section, help_screen_shows_shift_enter_when_kitty_protocol_active, chat_shows_pending_plan_banner_only_while_awaiting_approval, chat_message_thinking_block_toggles_between_collapsed_and_expanded, chat_message_perf_footer_reports_cold_and_warm_starts, chat_shows_streaming_response_and_processing_indicator, plan_mode_shows_full_document_with_tasks_and_criteria, plan_mode_shows_empty_state_without_a_plan, approval_dialog_shows_tool_name_capabilities_and_summarized_params, approval_dialog_details_view_shows_pretty_printed_json

### skills_view_shows_empty_state_message

*Rust Function* — `src/tui/render.rs#L2247-L2253`

_private_

```
async fn skills_view_shows_empty_state_message()
```

**Calls:** render_to_string

### skills_view_shows_name_and_description

*Rust Function* — `src/tui/render.rs#L2232-L2244`

_private_

```
async fn skills_view_shows_name_and_description()
```

**Calls:** render_to_string

### status_bar_shows_full_auto_when_active

*Rust Function* — `src/tui/render.rs#L2213-L2229`

_private_

```
async fn status_bar_shows_full_auto_when_active()
```

**Calls:** set_auto_mode_state, render_to_string

### status_bar_shows_interactive_by_default

*Rust Function* — `src/tui/render.rs#L2204-L2210`

_private_

```
async fn status_bar_shows_interactive_by_default()
```

**Calls:** render_to_string

### test_app

*Rust Function* — `src/tui/render.rs#L2075-L2081`

_private_

```
async fn test_app() -> App
```

**Calls:** run_migrations

### test_approval_request

*Rust Function* — `src/tui/render.rs#L2580-L2594`

_private_

```
fn test_approval_request( tool_input: serde_json::Value, capabilities: Vec<String>, ) -> crate::tui::events::ToolApprovalRequest
```

**Called by:** approval_dialog_shows_tool_name_capabilities_and_summarized_params, approval_dialog_details_view_shows_pretty_printed_json

### calculate_cost

*Rust Method* — `src/tui/runner.rs#L182-L184`

_private_

```
fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64
```

### complete

*Rust Method* — `src/tui/runner.rs#L164-L166`

_private_

```
async fn complete(&self, _request: LLMRequest) -> ProviderResult<LLMResponse>
```

### context_window

*Rust Method* — `src/tui/runner.rs#L179-L181`

_private_

```
fn context_window(&self, _model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `src/tui/runner.rs#L173-L175`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `src/tui/runner.rs#L170-L172`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `src/tui/runner.rs#L167-L169`

_private_

```
async fn stream(&self, _request: LLMRequest) -> ProviderResult<ProviderStream>
```

### supported_models

*Rust Method* — `src/tui/runner.rs#L176-L178`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### run

*Rust Function* — `src/tui/runner.rs#L27-L79`

```
pub async fn run(mut app: App) -> Result<()>
```

**Calls:** set_kitty_keyboard_protocol_active, run_inner

### run_inner

*Rust Function* — `src/tui/runner.rs#L84-L94`

_private_

```
async fn run_inner(stdout: io::Stdout, app: &mut App) -> Result<()>
```

**Calls:** initialize, event_sender, start_terminal_listener, run_loop

**Called by:** run

### run_loop

*Rust Function* — `src/tui/runner.rs#L97-L145`

_private_

```
async fn run_loop<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> where <B as Backend>::Error: std::error::Error + Send + Sync + 'static,
```

**Calls:** render, next_event, handle_event, try_next_event

**Called by:** run_inner, run_loop_exits_immediately_when_should_quit_is_set

### run_loop_exits_immediately_when_should_quit_is_set

*Rust Function* — `src/tui/runner.rs#L188-L202`

_private_

```
async fn run_loop_exits_immediately_when_should_quit_is_set()
```

**Calls:** run_migrations, run_loop

### render_splash

*Rust Function* — `src/tui/splash.rs#L14-L35`

```
pub fn render_splash(f: &mut Frame, area: Rect, provider_name: &str, model_name: &str)
```

**Calls:** render_splash_content

**Called by:** render

### render_splash_content

*Rust Function* — `src/tui/splash.rs#L37-L170`

_private_

```
fn render_splash_content(f: &mut Frame, area: Rect, provider_name: &str, model_name: &str)
```

**Calls:** block

**Called by:** render_splash

### api

*Rust Method* — `src/utils/retry.rs#L71-L73`

```
pub fn api() -> Self
```

**Called by:** test_preset_configs

### api_aggressive

*Rust Method* — `src/utils/retry.rs#L76-L84`

```
pub fn api_aggressive() -> Self
```

### calculate_delay

*Rust Method* — `src/utils/retry.rs#L95-L110`

```
pub fn calculate_delay(&self, attempt: u32) -> Duration
```

### database

*Rust Method* — `src/utils/retry.rs#L49-L57`

```
pub fn database() -> Self
```

**Called by:** test_preset_configs

### database_aggressive

*Rust Method* — `src/utils/retry.rs#L60-L68`

```
pub fn database_aggressive() -> Self
```

### default

*Rust Method* — `src/utils/retry.rs#L36-L44`

_private_

```
fn default() -> Self
```

### no_retry

*Rust Method* — `src/utils/retry.rs#L87-L92`

```
pub fn no_retry() -> Self
```

### retry_after

*Rust Method* — `src/utils/retry.rs#L15-L17`

_private_

```
fn retry_after(&self) -> Option<Duration>
```

**Called by:** retry

### is_retryable

*Rust Method* — `src/utils/retry.rs#L247-L249`

_private_

```
fn is_retryable(&self) -> bool
```

### fmt

*Rust Method* — `src/utils/retry.rs#L241-L243`

_private_

```
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

### retry

*Rust Function* — `src/utils/retry.rs#L120-L174`

```
pub async fn retry<F, Fut, T, E>( mut operation: F, config: &RetryConfig, ) -> std::result::Result<T, E> where F: FnMut() -> Fut, Fut: Future<Output = std::result::Result<T, E>>, E: RetryableError,
```

**Calls:** retry_after

**Called by:** test_successful_operation_no_retry, test_non_retryable_error_fails_immediately, test_retryable_error_retries, test_max_attempts_exceeded, test_no_retry_config

### retry_with_check

*Rust Function* — `src/utils/retry.rs#L179-L226`

```
pub async fn retry_with_check<F, Fut, T, E, C>( mut operation: F, config: &RetryConfig, is_retryable: C, ) -> std::result::Result<T, E> where F: FnMut() -> Fut, Fut: Future<Output = std::result::Result<T, E>>, E: std::fmt::Display, C: Fn(&E) -> bool,
```

### test_calculate_delay_capped

*Rust Function* — `src/utils/retry.rs#L394-L408`

_private_

```
fn test_calculate_delay_capped()
```

### test_calculate_delay_exponential

*Rust Function* — `src/utils/retry.rs#L378-L391`

_private_

```
fn test_calculate_delay_exponential()
```

### test_max_attempts_exceeded

*Rust Function* — `src/utils/retry.rs#L321-L350`

_private_

```
async fn test_max_attempts_exceeded()
```

**Calls:** retry

### test_no_retry_config

*Rust Function* — `src/utils/retry.rs#L353-L375`

_private_

```
async fn test_no_retry_config()
```

**Calls:** retry

### test_non_retryable_error_fails_immediately

*Rust Function* — `src/utils/retry.rs#L260-L282`

_private_

```
async fn test_non_retryable_error_fails_immediately()
```

**Calls:** retry

### test_preset_configs

*Rust Function* — `src/utils/retry.rs#L411-L423`

_private_

```
fn test_preset_configs()
```

**Calls:** database, api

### test_retryable_error_retries

*Rust Function* — `src/utils/retry.rs#L285-L318`

_private_

```
async fn test_retryable_error_retries()
```

**Calls:** retry

### test_successful_operation_no_retry

*Rust Function* — `src/utils/retry.rs#L253-L257`

_private_

```
async fn test_successful_operation_no_retry()
```

**Calls:** retry

### truncate_ascii_at_exact_boundary

*Rust Function* — `src/utils/mod.rs#L26-L29`

_private_

```
fn truncate_ascii_at_exact_boundary()
```

### truncate_at_char_boundary

*Rust Function* — `src/utils/mod.rs#L10-L19`

```
pub fn truncate_at_char_boundary(s: &str, max_bytes: usize) -> &str
```

**Calls:** len

**Called by:** augment_message_with_pdf, end_session_with_summary

### truncate_empty_and_zero

*Rust Function* — `src/utils/mod.rs#L41-L44`

_private_

```
fn truncate_empty_and_zero()
```

### truncate_multibyte_does_not_panic

*Rust Function* — `src/utils/mod.rs#L32-L38`

_private_

```
fn truncate_multibyte_does_not_panic()
```

### test_cli_db_invalid_operation

*Rust Function* — `tests/cli_test.rs#L270-L273`

_private_

```
fn test_cli_db_invalid_operation()
```

### test_cli_db_missing_operation

*Rust Function* — `tests/cli_test.rs#L264-L267`

_private_

```
fn test_cli_db_missing_operation()
```

### test_cli_invalid_format

*Rust Function* — `tests/cli_test.rs#L246-L249`

_private_

```
fn test_cli_invalid_format()
```

### test_cli_invalid_subcommand

*Rust Function* — `tests/cli_test.rs#L258-L261`

_private_

```
fn test_cli_invalid_subcommand()
```

### test_cli_missing_prompt_for_run

*Rust Function* — `tests/cli_test.rs#L252-L255`

_private_

```
fn test_cli_missing_prompt_for_run()
```

### test_cli_parse_chat_command

*Rust Function* — `tests/cli_test.rs#L18-L26`

_private_

```
fn test_cli_parse_chat_command()
```

### test_cli_parse_chat_with_session

*Rust Function* — `tests/cli_test.rs#L29-L37`

_private_

```
fn test_cli_parse_chat_with_session()
```

### test_cli_parse_combined_flags

*Rust Function* — `tests/cli_test.rs#L214-L243`

_private_

```
fn test_cli_parse_combined_flags()
```

### test_cli_parse_config_command

*Rust Function* — `tests/cli_test.rs#L146-L154`

_private_

```
fn test_cli_parse_config_command()
```

### test_cli_parse_config_path

*Rust Function* — `tests/cli_test.rs#L202-L205`

_private_

```
fn test_cli_parse_config_path()
```

### test_cli_parse_config_path_short

*Rust Function* — `tests/cli_test.rs#L208-L211`

_private_

```
fn test_cli_parse_config_path_short()
```

### test_cli_parse_config_with_show_secrets

*Rust Function* — `tests/cli_test.rs#L157-L165`

_private_

```
fn test_cli_parse_config_with_show_secrets()
```

### test_cli_parse_db_init

*Rust Function* — `tests/cli_test.rs#L168-L176`

_private_

```
fn test_cli_parse_db_init()
```

### test_cli_parse_db_stats

*Rust Function* — `tests/cli_test.rs#L179-L187`

_private_

```
fn test_cli_parse_db_stats()
```

### test_cli_parse_debug_flag

*Rust Function* — `tests/cli_test.rs#L190-L193`

_private_

```
fn test_cli_parse_debug_flag()
```

### test_cli_parse_debug_flag_short

*Rust Function* — `tests/cli_test.rs#L196-L199`

_private_

```
fn test_cli_parse_debug_flag_short()
```

### test_cli_parse_init_command

*Rust Function* — `tests/cli_test.rs#L124-L132`

_private_

```
fn test_cli_parse_init_command()
```

### test_cli_parse_init_with_force

*Rust Function* — `tests/cli_test.rs#L135-L143`

_private_

```
fn test_cli_parse_init_with_force()
```

### test_cli_parse_no_command

*Rust Function* — `tests/cli_test.rs#L9-L15`

_private_

```
fn test_cli_parse_no_command()
```

### test_cli_parse_run_command

*Rust Function* — `tests/cli_test.rs#L40-L54`

_private_

```
fn test_cli_parse_run_command()
```

### test_cli_parse_run_with_auto_approve

*Rust Function* — `tests/cli_test.rs#L92-L105`

_private_

```
fn test_cli_parse_run_with_auto_approve()
```

### test_cli_parse_run_with_json_format

*Rust Function* — `tests/cli_test.rs#L57-L71`

_private_

```
fn test_cli_parse_run_with_json_format()
```

### test_cli_parse_run_with_markdown_format

*Rust Function* — `tests/cli_test.rs#L74-L89`

_private_

```
fn test_cli_parse_run_with_markdown_format()
```

### test_cli_parse_run_with_yolo_alias

*Rust Function* — `tests/cli_test.rs#L108-L121`

_private_

```
fn test_cli_parse_run_with_yolo_alias()
```

### create_pool_with_schema

*Rust Function* — `tests/codebase_index_test.rs#L8-L24`

_private_

```
async fn create_pool_with_schema() -> sqlx::SqlitePool
```

**Called by:** index_and_query_provider_trait, index_file_twice_no_duplicate, fts_search_finds_symbol_by_partial_name, index_nonexistent_file_returns_error

### fts_search_finds_symbol_by_partial_name

*Rust Function* — `tests/codebase_index_test.rs#L81-L101`

_private_

```
async fn fts_search_finds_symbol_by_partial_name()
```

**Calls:** create_pool_with_schema, index_file, fts_search

### index_and_query_provider_trait

*Rust Function* — `tests/codebase_index_test.rs#L30-L56`

_private_

```
async fn index_and_query_provider_trait()
```

**Calls:** create_pool_with_schema, index_file, query_symbol

### index_file_twice_no_duplicate

*Rust Function* — `tests/codebase_index_test.rs#L60-L77`

_private_

```
async fn index_file_twice_no_duplicate()
```

**Calls:** create_pool_with_schema, index_file, query_symbol

### index_nonexistent_file_returns_error

*Rust Function* — `tests/codebase_index_test.rs#L105-L114`

_private_

```
async fn index_nonexistent_file_returns_error()
```

**Calls:** create_pool_with_schema, index_file

### build_context

*Rust Function* — `tests/compaction_test.rs#L39-L53`

_private_

```
fn build_context(session_id: Uuid, n: usize, max_tokens: usize) -> AgentContext
```

**Calls:** add_message, text_message

**Called by:** compaction_preserves_last_10_turns, compaction_fails_gracefully_with_insufficient_turns, compaction_writes_one_record_to_db

### compaction_fails_gracefully_with_insufficient_turns

*Rust Function* — `tests/compaction_test.rs#L114-L136`

_private_

```
async fn compaction_fails_gracefully_with_insufficient_turns()
```

**Calls:** run_migrations, build_context, len, compact

### compaction_preserves_last_10_turns

*Rust Function* — `tests/compaction_test.rs#L57-L106`

_private_

```
async fn compaction_preserves_last_10_turns()
```

**Calls:** run_migrations, build_context, compact

### compaction_writes_one_record_to_db

*Rust Function* — `tests/compaction_test.rs#L156-L171`

_private_

```
async fn compaction_writes_one_record_to_db()
```

**Calls:** run_migrations, build_context, compact, list_for_session

### create_session

*Rust Function* — `tests/compaction_test.rs#L12-L27`

_private_

```
async fn create_session(pool: &sqlx::SqlitePool, session_id: Uuid)
```

### should_compact_fires_at_80_percent

*Rust Function* — `tests/compaction_test.rs#L140-L152`

_private_

```
fn should_compact_fires_at_80_percent()
```

### text_message

*Rust Function* — `tests/compaction_test.rs#L29-L36`

_private_

```
fn text_message(role: Role, text: &str) -> Message
```

**Called by:** build_context

### new

*Rust Method* — `tests/error_scenarios_test.rs#L38-L40`

_private_

```
fn new(error_type: ErrorType) -> Self
```

### calculate_cost

*Rust Method* — `tests/error_scenarios_test.rs#L83-L85`

_private_

```
fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64
```

### complete

*Rust Method* — `tests/error_scenarios_test.rs#L45-L61`

_private_

```
async fn complete(&self, _request: LLMRequest) -> ProviderResult<LLMResponse>
```

### context_window

*Rust Method* — `tests/error_scenarios_test.rs#L79-L81`

_private_

```
fn context_window(&self, _model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `tests/error_scenarios_test.rs#L71-L73`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `tests/error_scenarios_test.rs#L67-L69`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `tests/error_scenarios_test.rs#L63-L65`

_private_

```
async fn stream(&self, _request: LLMRequest) -> ProviderResult<ProviderStream>
```

### supported_models

*Rust Method* — `tests/error_scenarios_test.rs#L75-L77`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### calculate_cost

*Rust Method* — `tests/error_scenarios_test.rs#L359-L361`

_private_

```
fn calculate_cost(&self, _model: &str, input: u32, output: u32) -> f64
```

### complete

*Rust Method* — `tests/error_scenarios_test.rs#L322-L337`

_private_

```
async fn complete(&self, _request: LLMRequest) -> ProviderResult<LLMResponse>
```

### context_window

*Rust Method* — `tests/error_scenarios_test.rs#L355-L357`

_private_

```
fn context_window(&self, _model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `tests/error_scenarios_test.rs#L347-L349`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `tests/error_scenarios_test.rs#L343-L345`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `tests/error_scenarios_test.rs#L339-L341`

_private_

```
async fn stream(&self, _request: LLMRequest) -> ProviderResult<ProviderStream>
```

### supported_models

*Rust Method* — `tests/error_scenarios_test.rs#L351-L353`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### create_error_agent

*Rust Function* — `tests/error_scenarios_test.rs#L94-L110`

_private_

```
async fn create_error_agent( db: &Database, error_type: ErrorType, ) -> Result<(AgentService, ServiceContext)>
```

**Calls:** register, with_tool_registry

**Called by:** test_error_api_error, test_error_rate_limit, test_error_timeout, test_error_invalid_response, test_error_authentication, test_error_session_not_found, test_error_recovery_after_failure

### create_test_db

*Rust Function* — `tests/error_scenarios_test.rs#L88-L92`

_private_

```
async fn create_test_db() -> Result<Database>
```

**Calls:** run_migrations

### test_error_api_error

*Rust Function* — `tests/error_scenarios_test.rs#L113-L130`

_private_

```
async fn test_error_api_error() -> Result<()>
```

**Calls:** create_error_agent

### test_error_authentication

*Rust Function* — `tests/error_scenarios_test.rs#L195-L217`

_private_

```
async fn test_error_authentication() -> Result<()>
```

**Calls:** create_error_agent

### test_error_database_concurrent_access

*Rust Function* — `tests/error_scenarios_test.rs#L261-L287`

_private_

```
async fn test_error_database_concurrent_access() -> Result<()>
```

**Calls:** get_session

### test_error_empty_message

*Rust Function* — `tests/error_scenarios_test.rs#L235-L258`

_private_

```
async fn test_error_empty_message() -> Result<()>
```

### test_error_invalid_response

*Rust Function* — `tests/error_scenarios_test.rs#L174-L192`

_private_

```
async fn test_error_invalid_response() -> Result<()>
```

**Calls:** create_error_agent

### test_error_rate_limit

*Rust Function* — `tests/error_scenarios_test.rs#L133-L150`

_private_

```
async fn test_error_rate_limit() -> Result<()>
```

**Calls:** create_error_agent

### test_error_recovery_after_failure

*Rust Function* — `tests/error_scenarios_test.rs#L290-L315`

_private_

```
async fn test_error_recovery_after_failure() -> Result<()>
```

**Calls:** create_error_agent, get_session

### test_error_session_not_found

*Rust Function* — `tests/error_scenarios_test.rs#L220-L232`

_private_

```
async fn test_error_session_not_found() -> Result<()>
```

**Calls:** create_error_agent

### test_error_timeout

*Rust Function* — `tests/error_scenarios_test.rs#L153-L171`

_private_

```
async fn test_error_timeout() -> Result<()>
```

**Calls:** create_error_agent

### new

*Rust Method* — `tests/integration_test.rs#L30-L35`

_private_

```
fn new(responses: Vec<String>) -> Self
```

### calculate_cost

*Rust Method* — `tests/integration_test.rs#L88-L91`

_private_

```
fn calculate_cost(&self, _model: &str, input_tokens: u32, output_tokens: u32) -> f64
```

### complete

*Rust Method* — `tests/integration_test.rs#L44-L71`

_private_

```
async fn complete( &self, _request: LLMRequest, ) -> crustly::llm::provider::error::Result<LLMResponse>
```

**Calls:** len

### context_window

*Rust Method* — `tests/integration_test.rs#L101-L103`

_private_

```
fn context_window(&self, _model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `tests/integration_test.rs#L93-L95`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `tests/integration_test.rs#L80-L82`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `tests/integration_test.rs#L73-L78`

_private_

```
async fn stream( &self, _request: LLMRequest, ) -> crustly::llm::provider::error::Result<ProviderStream>
```

### supported_models

*Rust Method* — `tests/integration_test.rs#L97-L99`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### supports_streaming

*Rust Method* — `tests/integration_test.rs#L84-L86`

_private_

```
fn supports_streaming(&self) -> bool
```

### single_response

*Rust Method* — `tests/integration_test.rs#L37-L39`

_private_

```
fn single_response(response: String) -> Self
```

**Called by:** test_end_to_end_system_prompt

### create_test_agent

*Rust Function* — `tests/integration_test.rs#L114-L130`

_private_

```
async fn create_test_agent( db: &Database, responses: Vec<String>, ) -> Result<(AgentService, ServiceContext)>
```

**Calls:** register, with_tool_registry

**Called by:** test_end_to_end_simple_message, test_end_to_end_multi_turn_conversation, test_end_to_end_session_management, test_end_to_end_cost_tracking, test_end_to_end_error_handling, test_end_to_end_token_usage

### create_test_db

*Rust Function* — `tests/integration_test.rs#L107-L111`

_private_

```
async fn create_test_db() -> Result<Database>
```

**Calls:** run_migrations

### test_config_loading

*Rust Function* — `tests/integration_test.rs#L411-L423`

_private_

```
async fn test_config_loading() -> Result<()>
```

### test_database_persistence

*Rust Function* — `tests/integration_test.rs#L426-L463`

_private_

```
async fn test_database_persistence() -> Result<()>
```

**Calls:** run_migrations, drop, get_session

### test_end_to_end_cost_tracking

*Rust Function* — `tests/integration_test.rs#L279-L323`

_private_

```
async fn test_end_to_end_cost_tracking() -> Result<()>
```

**Calls:** create_test_agent, get_session

### test_end_to_end_error_handling

*Rust Function* — `tests/integration_test.rs#L326-L342`

_private_

```
async fn test_end_to_end_error_handling() -> Result<()>
```

**Calls:** create_test_agent

### test_end_to_end_multi_turn_conversation

*Rust Function* — `tests/integration_test.rs#L172-L219`

_private_

```
async fn test_end_to_end_multi_turn_conversation() -> Result<()>
```

**Calls:** create_test_agent, list_messages_for_session

### test_end_to_end_session_management

*Rust Function* — `tests/integration_test.rs#L222-L276`

_private_

```
async fn test_end_to_end_session_management() -> Result<()>
```

**Calls:** create_test_agent, list_messages_for_session, list_sessions

### test_end_to_end_simple_message

*Rust Function* — `tests/integration_test.rs#L133-L169`

_private_

```
async fn test_end_to_end_simple_message() -> Result<()>
```

**Calls:** create_test_agent, list_messages_for_session

### test_end_to_end_system_prompt

*Rust Function* — `tests/integration_test.rs#L383-L408`

_private_

```
async fn test_end_to_end_system_prompt() -> Result<()>
```

**Calls:** single_response

### test_end_to_end_token_usage

*Rust Function* — `tests/integration_test.rs#L345-L380`

_private_

```
async fn test_end_to_end_token_usage() -> Result<()>
```

**Calls:** create_test_agent, list_messages_for_session, get_session

### connect_times_out_gracefully

*Rust Function* — `tests/mcp_contract_test.rs#L61-L80`

_private_

```
async fn connect_times_out_gracefully()
```

### connect_to_nonexistent_server_returns_error

*Rust Function* — `tests/mcp_contract_test.rs#L10-L24`

_private_

```
async fn connect_to_nonexistent_server_returns_error()
```

### unhealthy_client_returns_graceful_error

*Rust Function* — `tests/mcp_contract_test.rs#L29-L54`

_private_

```
async fn unhealthy_client_returns_graceful_error()
```

**Calls:** call_tool

### all_tiers_resolve_to_valid_model

*Rust Function* — `tests/model_routing_test.rs#L10-L17`

_private_

```
fn all_tiers_resolve_to_valid_model()
```

### complex_prompt_routes_to_powerful_tier

*Rust Function* — `tests/model_routing_test.rs#L56-L73`

_private_

```
fn complex_prompt_routes_to_powerful_tier()
```

**Calls:** classify_tier

### neutral_prompt_routes_to_balanced_tier

*Rust Function* — `tests/model_routing_test.rs#L97-L102`

_private_

```
fn neutral_prompt_routes_to_balanced_tier()
```

**Calls:** classify_tier

### simple_prompt_routes_to_fast_tier

*Rust Function* — `tests/model_routing_test.rs#L77-L93`

_private_

```
fn simple_prompt_routes_to_fast_tier()
```

**Calls:** classify_tier

### thinking_config_forces_temperature_one

*Rust Function* — `tests/model_routing_test.rs#L21-L27`

_private_

```
fn thinking_config_forces_temperature_one()
```

### token_limits_ordered_by_tier

*Rust Function* — `tests/model_routing_test.rs#L41-L52`

_private_

```
fn token_limits_ordered_by_tier()
```

### zero_budget_does_not_enable_thinking

*Rust Function* — `tests/model_routing_test.rs#L31-L37`

_private_

```
fn zero_budget_does_not_enable_thinking()
```

### advance_transitions_through_tasks_to_done

*Rust Function* — `tests/plan_autorun_test.rs#L114-L141`

_private_

```
fn advance_transitions_through_tasks_to_done()
```

**Calls:** advance

### auto_plan_approval_goes_to_auto_executing

*Rust Function* — `tests/plan_autorun_test.rs#L32-L65`

_private_

```
fn auto_plan_approval_goes_to_auto_executing()
```

### high_risk_tools_pause_auto_execution

*Rust Function* — `tests/plan_autorun_test.rs#L83-L110`

_private_

```
fn high_risk_tools_pause_auto_execution()
```

### interactive_approval_goes_to_executing

*Rust Function* — `tests/plan_autorun_test.rs#L69-L79`

_private_

```
fn interactive_approval_goes_to_executing()
```

### make_plan_task

*Rust Function* — `tests/plan_autorun_test.rs#L8-L27`

_private_

```
fn make_plan_task(order: usize, title: &str) -> PlanTask
```

### crash_recovery_resumes_at_correct_task

*Rust Function* — `tests/plan_crash_recovery_test.rs#L69-L126`

_private_

```
async fn crash_recovery_resumes_at_correct_task()
```

**Calls:** run_migrations, create_plan, create_task, minimal_task

### create_plan

*Rust Function* — `tests/plan_crash_recovery_test.rs#L26-L42`

_private_

```
async fn create_plan(pool: &sqlx::SqlitePool, plan_id: Uuid, session_id: Uuid)
```

**Called by:** crash_recovery_resumes_at_correct_task, task_state_transitions_correct_order, failed_task_stores_error_without_completion_timestamp

### create_session

*Rust Function* — `tests/plan_crash_recovery_test.rs#L10-L24`

_private_

```
async fn create_session(pool: &sqlx::SqlitePool, session_id: Uuid)
```

### failed_task_stores_error_without_completion_timestamp

*Rust Function* — `tests/plan_crash_recovery_test.rs#L175-L208`

_private_

```
async fn failed_task_stores_error_without_completion_timestamp()
```

**Calls:** run_migrations, create_plan, minimal_task, create_task, update_task_status

### interrupted_plan_none_when_all_done

*Rust Function* — `tests/plan_crash_recovery_test.rs#L212-L220`

_private_

```
fn interrupted_plan_none_when_all_done()
```

**Calls:** interrupted_plan_from_tasks

### interrupted_plan_resumes_at_lowest_incomplete

*Rust Function* — `tests/plan_crash_recovery_test.rs#L224-L236`

_private_

```
fn interrupted_plan_resumes_at_lowest_incomplete()
```

**Calls:** interrupted_plan_from_tasks

### minimal_task

*Rust Function* — `tests/plan_crash_recovery_test.rs#L45-L63`

_private_

```
fn minimal_task(plan_id: Uuid, task_order: i32, status: &str) -> crustly::db::models::PlanTask
```

**Called by:** crash_recovery_resumes_at_correct_task, task_state_transitions_correct_order, failed_task_stores_error_without_completion_timestamp

### task_state_transitions_correct_order

*Rust Function* — `tests/plan_crash_recovery_test.rs#L130-L171`

_private_

```
async fn task_state_transitions_correct_order()
```

**Calls:** run_migrations, create_plan, minimal_task, create_task, update_task_status

### create_multi_task_plan

*Rust Function* — `tests/plan_mode_integration_test.rs#L44-L124`

_private_

```
fn create_multi_task_plan(session_id: Uuid) -> PlanDocument
```

**Calls:** add_task

**Called by:** test_end_to_end_plan_creation_and_retrieval, test_plan_state_transition_workflow, test_multiple_concurrent_plans_for_same_session, test_multiple_sessions_with_separate_plans, test_plan_deletion_with_cascade, test_json_export_import_integration, test_plan_rejection_workflow, test_task_blocking_and_failure_scenarios, test_get_most_recent_plan_integration

### setup_test_env

*Rust Function* — `tests/plan_mode_integration_test.rs#L18-L41`

_private_

```
async fn setup_test_env() -> (Database, ServiceContext, PlanService, Session, TempDir)
```

**Calls:** run_migrations

**Called by:** test_end_to_end_plan_creation_and_retrieval, test_plan_state_transition_workflow, test_multiple_concurrent_plans_for_same_session, test_multiple_sessions_with_separate_plans, test_plan_deletion_with_cascade, test_json_export_import_integration, test_plan_rejection_workflow, test_task_blocking_and_failure_scenarios, test_get_most_recent_plan_integration

### test_end_to_end_plan_creation_and_retrieval

*Rust Function* — `tests/plan_mode_integration_test.rs#L127-L161`

_private_

```
async fn test_end_to_end_plan_creation_and_retrieval()
```

**Calls:** setup_test_env, create_multi_task_plan

### test_get_most_recent_plan_integration

*Rust Function* — `tests/plan_mode_integration_test.rs#L434-L467`

_private_

```
async fn test_get_most_recent_plan_integration()
```

**Calls:** setup_test_env, get_most_recent_plan, create_multi_task_plan

### test_json_export_import_integration

*Rust Function* — `tests/plan_mode_integration_test.rs#L336-L374`

_private_

```
async fn test_json_export_import_integration()
```

**Calls:** setup_test_env, create_multi_task_plan, export_to_json, import_from_json

### test_multiple_concurrent_plans_for_same_session

*Rust Function* — `tests/plan_mode_integration_test.rs#L221-L262`

_private_

```
async fn test_multiple_concurrent_plans_for_same_session()
```

**Calls:** setup_test_env, create_multi_task_plan

### test_multiple_sessions_with_separate_plans

*Rust Function* — `tests/plan_mode_integration_test.rs#L265-L307`

_private_

```
async fn test_multiple_sessions_with_separate_plans()
```

**Calls:** setup_test_env, create_multi_task_plan

### test_plan_deletion_with_cascade

*Rust Function* — `tests/plan_mode_integration_test.rs#L310-L333`

_private_

```
async fn test_plan_deletion_with_cascade()
```

**Calls:** setup_test_env, create_multi_task_plan

### test_plan_rejection_workflow

*Rust Function* — `tests/plan_mode_integration_test.rs#L377-L394`

_private_

```
async fn test_plan_rejection_workflow()
```

**Calls:** setup_test_env, create_multi_task_plan

### test_plan_state_transition_workflow

*Rust Function* — `tests/plan_mode_integration_test.rs#L164-L218`

_private_

```
async fn test_plan_state_transition_workflow()
```

**Calls:** setup_test_env, create_multi_task_plan, get_task_mut

### test_task_blocking_and_failure_scenarios

*Rust Function* — `tests/plan_mode_integration_test.rs#L397-L431`

_private_

```
async fn test_task_blocking_and_failure_scenarios()
```

**Calls:** setup_test_env, create_multi_task_plan, get_task_mut

### new

*Rust Method* — `tests/streaming_test.rs#L23-L74`

_private_

```
fn new(text_chunks: Vec<&str>) -> Self
```

### calculate_cost

*Rust Method* — `tests/streaming_test.rs#L114-L116`

_private_

```
fn calculate_cost(&self, _model: &str, input_tokens: u32, output_tokens: u32) -> f64
```

### complete

*Rust Method* — `tests/streaming_test.rs#L87-L90`

_private_

```
async fn complete(&self, _request: LLMRequest) -> ProviderResult<LLMResponse>
```

### context_window

*Rust Method* — `tests/streaming_test.rs#L110-L112`

_private_

```
fn context_window(&self, _model: &str) -> Option<u32>
```

### default_model

*Rust Method* — `tests/streaming_test.rs#L102-L104`

_private_

```
fn default_model(&self) -> &str
```

### name

*Rust Method* — `tests/streaming_test.rs#L98-L100`

_private_

```
fn name(&self) -> &str
```

### stream

*Rust Method* — `tests/streaming_test.rs#L92-L96`

_private_

```
async fn stream(&self, _request: LLMRequest) -> ProviderResult<ProviderStream>
```

### supported_models

*Rust Method* — `tests/streaming_test.rs#L106-L108`

_private_

```
fn supported_models(&self) -> Vec<String>
```

### supports_streaming

*Rust Method* — `tests/streaming_test.rs#L118-L120`

_private_

```
fn supports_streaming(&self) -> bool
```

### with_error

*Rust Method* — `tests/streaming_test.rs#L76-L82`

_private_

```
fn with_error(error_message: &str) -> Self
```

**Called by:** test_streaming_error_handling

### test_provider_supports_streaming

*Rust Function* — `tests/streaming_test.rs#L338-L341`

_private_

```
async fn test_provider_supports_streaming()
```

### test_streaming_basic

*Rust Function* — `tests/streaming_test.rs#L124-L158`

_private_

```
async fn test_streaming_basic() -> Result<()>
```

**Calls:** with_streaming, next

### test_streaming_content_accumulation

*Rust Function* — `tests/streaming_test.rs#L308-L327`

_private_

```
async fn test_streaming_content_accumulation() -> Result<()>
```

**Calls:** with_streaming, next

### test_streaming_empty_response

*Rust Function* — `tests/streaming_test.rs#L289-L305`

_private_

```
async fn test_streaming_empty_response() -> Result<()>
```

**Calls:** with_streaming, next

### test_streaming_error_handling

*Rust Function* — `tests/streaming_test.rs#L269-L286`

_private_

```
async fn test_streaming_error_handling() -> Result<()>
```

**Calls:** with_error, with_streaming, next

### test_streaming_multiple_chunks

*Rust Function* — `tests/streaming_test.rs#L186-L215`

_private_

```
async fn test_streaming_multiple_chunks() -> Result<()>
```

**Calls:** with_streaming, next

### test_streaming_request_builder

*Rust Function* — `tests/streaming_test.rs#L330-L335`

_private_

```
async fn test_streaming_request_builder()
```

**Calls:** with_streaming

### test_streaming_single_chunk

*Rust Function* — `tests/streaming_test.rs#L161-L183`

_private_

```
async fn test_streaming_single_chunk() -> Result<()>
```

**Calls:** with_streaming, next

### test_streaming_stop_reason

*Rust Function* — `tests/streaming_test.rs#L251-L266`

_private_

```
async fn test_streaming_stop_reason() -> Result<()>
```

**Calls:** with_streaming, next

### test_streaming_token_counting

*Rust Function* — `tests/streaming_test.rs#L218-L248`

_private_

```
async fn test_streaming_token_counting() -> Result<()>
```

**Calls:** with_streaming, next

## Interfaces

### PoolExt

*Rust Trait* — `src/db/mod.rs#L186-L199`

```
pub trait PoolExt
```

### Repository

*Rust Trait* — `src/db/repository/mod.rs#L23-L38`

```
pub trait Repository<T>
```

### Provider

*Rust Trait* — `src/llm/provider/trait.rs#L19-L65`

```
pub trait Provider: Send + Sync
```

### PermissionPolicy

*Rust Trait* — `src/llm/tools/sandbox.rs#L38-L40`

```
pub trait PermissionPolicy: Send + Sync
```

### SubAgentLauncher

*Rust Trait* — `src/llm/tools/trait.rs#L17-L26`

```
pub trait SubAgentLauncher: Send + Sync + std::fmt::Debug
```

### Tool

*Rust Trait* — `src/llm/tools/trait.rs#L183-L218`

```
pub trait Tool: Send + Sync
```

### RetryableError

*Rust Trait* — `src/utils/retry.rs#L10-L18`

```
pub trait RetryableError: std::fmt::Display
```

## Modules

### database

*Rust Module* — `benches/database.rs#L1-L326`

**Imports:** external/criterion-black-box-criterion-group-criterion-main-benchmarkid-criterion, external/crustly-db-models-session-database, external/tempfile-tempdir

**Member of:** crustly

### parallel_tool_dispatch

*Rust Module* — `benches/parallel_tool_dispatch.rs#L1-L64`

**Imports:** external/criterion-criterion-group-criterion-main-criterion, external/std-time-duration, external/tempfile-tempdir, external/tokio-runtime-runtime

**Member of:** crustly

### src

*Rust Module* — `src/lib.rs#L1-L61`

**Imports:** external/pub-use-error-crustlyerror-errorcode

**Member of:** crustly

### src

*Rust Module* — `src/main.rs#L1-L27`

**Imports:** external/anyhow-result, external/clap-parser, external/crustly-cli-logging

**Member of:** crustly

### app

*Rust Module* — `src/app/mod.rs#L1-L106`

**Imports:** external/anyhow-result, external/notify-event-eventkind-recursivemode-watcher, external/sqlx-sqlitepool, external/std-path-path-pathbuf, external/std-sync-arc

**Member of:** crustly

### cli

*Rust Module* — `src/cli/mod.rs#L1-L1583`

**Imports:** external/anyhow-context-result, external/clap-parser-subcommand, external/std-sync-arc, external/crate-config-config, external/crate-db-database, external/std-io-self-write, external/crate-llm-tools-agent-agenttool-apply-patch-applypatchtool-ask-user-askusertool-bash-bashtool-code-exec-codeexectool-context-contexttool-doc-parser-docparsertool-edit-edittool-glob-globtool-grep-greptool-http-httpclienttool-ls-lstool-notebook-notebookedittool-plan-tool-plantool-powershell-powershelltool-read-readtool-registry-toolregistry-save-memory-savememorytool-skill-skilltool-task-tasktool-todo-write-todowritetool-web-fetch-webfetchtool-web-search-websearchtool-write-writetool, external/crate-tui-events-toolapprovalrequest-tuievent, external/tokio-sync-mpsc, external/crate-db-database-llm-agent-agentservice-services-servicecontext-tui, external/crate-config-planexecmode, external/crate-plan-planmodestate, external/crate-db-database-llm-agent-agentservice-services-servicecontext-sessionservice, external/crate-config-secrets-secretstring, external/crate-llm-provider-ollama-models, external/std-io-write-as, external/crate-logging, external/std-io-bufread-bufreader, external/super, external/clap-commandfactory

**Member of:** crustly

### config

*Rust Module* — `src/config/mod.rs#L1-L1476`

**Imports:** external/pub-use-crabrace-crabraceconfig-crabraceintegration, external/pub-use-secrets-providersecrets-secretstring, external/pub-use-update-providerupdater-updateresult, external/anyhow-context-result, external/serde-deserialize-serialize, external/std-fs, external/std-path-path-pathbuf, external/crate-llm-tools-sandbox-allowall-andpolicy-bashcommandallowlist-denypathprefixrule-denytoolrule, external/super, external/tempfile-namedtempfile, external/crate-llm-tools-sandbox-policydecision

**Member of:** crustly

### crabrace

*Rust Module* — `src/config/crabrace.rs#L1-L144`

**Imports:** external/anyhow-context-result, external/crabrace-crabraceclient-provider, external/serde-deserialize-serialize, external/super

**Member of:** crustly

### secrets

*Rust Module* — `src/config/secrets.rs#L1-L393`

**Imports:** external/anyhow-context-result, external/keyring-entry, external/serde-deserialize-serialize, external/std-fmt, external/zeroize-zeroize-zeroizeondrop, external/super

**Member of:** crustly

### update

*Rust Module* — `src/config/update.rs#L1-L277`

**Imports:** external/anyhow-context-result, external/crabrace-provider, external/std-time-duration-systemtime, external/tokio-time, external/tracing-debug-info-warn, external/super-crabrace-crabraceintegration, external/super-config-providerconfig, external/super, external/crate-config-crabrace-crabraceconfig

**Member of:** crustly

### db

*Rust Module* — `src/db/mod.rs#L1-L440`

**Imports:** external/pub-use-models, external/pub-use-repository, external/pub-use-retry-retry-db-anyhow-retry-db-operation-retry-db-sqlx-dbretryconfig, external/anyhow-context-result, external/sqlx-sqlite-sqlitepooloptions-sqlitepool, external/std-path-path, external/super

**Member of:** crustly

### models

*Rust Module* — `src/db/models.rs#L1-L446`

**Imports:** external/chrono-datetime-utc, external/serde-deserialize-serialize, external/sqlx-fromrow, external/uuid-uuid, external/sqlx-row, external/super

**Member of:** crustly

### repository

*Rust Module* — `src/db/repository/mod.rs#L1-L38`

**Imports:** external/pub-use-compaction-compactionrecordrepository, external/pub-use-file-filerepository, external/pub-use-memory-episodicmemoryrepository, external/pub-use-message-messagerepository, external/pub-use-plan-planrepository-plantaskrepository, external/pub-use-session-sessionlistoptions-sessionrepository, external/anyhow-result

**Member of:** crustly

### compaction

*Rust Module* — `src/db/repository/compaction.rs#L1-L65`

**Imports:** external/crate-db-models-compactionrecord, external/anyhow-result, external/chrono-datetime, external/sqlx-sqlitepool, external/uuid-uuid

**Member of:** crustly

### file

*Rust Module* — `src/db/repository/file.rs#L1-L239`

**Imports:** external/crate-db-models-file, external/anyhow-context-result, external/sqlx-sqlitepool, external/std-path-path, external/uuid-uuid, external/super, external/crate-db-models-session, external/crate-db-repository-sessionrepository, external/crate-db-database, external/std-path-pathbuf

**Member of:** crustly

### memory

*Rust Module* — `src/db/repository/memory.rs#L1-L254`

**Imports:** external/crate-llm-agent-memory-episodicmemory, external/crate-llm-provider-types-contentblock-message-role, external/anyhow-result, external/sqlx-sqlitepool, external/uuid-uuid, external/super, external/crate-llm-agent-context-agentcontext

**Member of:** crustly

### message

*Rust Module* — `src/db/repository/message.rs#L1-L306`

**Imports:** external/crate-db-models-message, external/anyhow-context-result, external/sqlx-sqlitepool, external/uuid-uuid, external/super, external/crate-db-models-session, external/crate-db-repository-sessionrepository, external/crate-db-database

**Member of:** crustly

### plan

*Rust Module* — `src/db/repository/plan.rs#L1-L1317`

**Imports:** external/crate-db-models-plan-plantask-plantaskstatus, external/crate-plan-plandocument-planstatus-taskstatus-tasktype, external/anyhow-context-result, external/sqlx-sqlitepool, external/uuid-uuid, external/chrono-datetime, external/super, external/crate-db-models-session, external/crate-db-repository-session-sessionrepository, external/crate-db-database, external/crate-plan-plantask-tasktype, external/chrono-utc

**Member of:** crustly

### session

*Rust Module* — `src/db/repository/session.rs#L1-L334`

**Imports:** external/crate-db-models-session, external/anyhow-context-result, external/chrono-utc, external/sqlx-sqlitepool, external/uuid-uuid, external/super, external/crate-db-database

**Member of:** crustly

### retry

*Rust Module* — `src/db/retry.rs#L1-L395`

**Imports:** external/anyhow-context-result, external/std-future-future, external/std-time-duration, external/tokio-time-sleep, external/super, external/sqlx-error, external/std-sync-atomic-atomicu32-ordering, external/std-sync-arc

**Member of:** crustly

### error

*Rust Module* — `src/error.rs#L1-L93`

**Imports:** external/thiserror-error

**Member of:** crustly

### events

*Rust Module* — `src/events/mod.rs#L1-L14`

**Member of:** crustly

### llm

*Rust Module* — `src/llm/mod.rs#L1-L18`

**Imports:** external/pub-use-provider-anthropicprovider-contentblock-llmrequest-llmresponse-message-provider-providererror-providerstream-role-stopreason-streamevent-tokenusage-tool, external/pub-use-agent-agentcontext-agenterror-agentservice, external/pub-use-tools-toolerror-toolregistry-toolresult

**Member of:** crustly

### agent

*Rust Module* — `src/llm/agent/mod.rs#L1-L17`

**Imports:** external/pub-use-context-agentcontext, external/pub-use-error-agenterror-result, external/pub-use-service-agentresponse-agentservice-agentstreamresponse-approvalcallback-toolapprovalinfo

**Member of:** crustly

### compaction

*Rust Module* — `src/llm/agent/compaction.rs#L1-L401`

**Imports:** external/crate-llm-agent-context-agentcontext, external/anyhow-result, external/chrono-datetime-utc, external/uuid-uuid, external/crate-llm-provider-types-contentblock-message-role, external/crate-llm-provider-types-contentblock, external/super

**Member of:** crustly

### context

*Rust Module* — `src/llm/agent/context.rs#L1-L357`

**Imports:** external/crate-db-models-message-as-dbmessage, external/crate-llm-provider-types-cachemetrics, external/crate-llm-provider-contentblock-message-role, external/std-path-pathbuf, external/uuid-uuid, external/crate-db-repository-episodicmemoryrepository, external/super

**Member of:** crustly

### error

*Rust Module* — `src/llm/agent/error.rs#L1-L47`

**Imports:** external/crate-llm-provider-providererror, external/thiserror-error

**Member of:** crustly

### memory

*Rust Module* — `src/llm/agent/memory.rs#L1-L196`

**Imports:** external/anyhow-result, external/chrono-datetime-utc, external/serde-deserialize-serialize, external/sqlx-sqlitepool, external/std-path-path, external/uuid-uuid

**Member of:** crustly

### service

*Rust Module* — `src/llm/agent/service.rs#L1-L2625`

**Imports:** external/super-context-agentcontext, external/super-error-agenterror-result, external/crate-llm-provider-router-modelrouter, external/crate-llm-provider-contentblock-contentdelta-llmrequest-llmresponse-message-perfmetrics-provider-providerstream-stopreason-streamevent-tokenusage, external/crate-llm-tools-cache-cachekey-toolresultcache-toolttlconfig, external/crate-llm-tools-filereadcache-toolcapability-toolexecutioncontext-toolregistry, external/crate-services-messageservice-servicecontext-sessionservice, external/futures-future-join-all, external/futures-streamext-as, external/serde-json-value, external/std-future-future, external/std-pin-pin, external/std-sync-arc, external/tokio-sync-mpsc, external/uuid-uuid, external/std-collections-hash-map-defaulthasher, external/std-hash-hash-hasher, external/crate-llm-provider-providererror, external/super, external/crate-db-database, external/crate-llm-provider-llmrequest-llmresponse-tokenusage, external/async-trait-async-trait, external/crate-llm-tools-subagentlauncher, external/crate-llm-provider-ollamaprovider-provider-tool, external/crate-llm-provider-types-messagedelta, external/crate-llm-provider-contentdelta

**Member of:** crustly

### pdf_context

*Rust Module* — `src/llm/pdf_context.rs#L1-L215`

**Imports:** external/std-path-path-pathbuf, external/super, external/std-io-write, external/tempfile-namedtempfile

**Member of:** crustly

### prompt

*Rust Module* — `src/llm/prompt/mod.rs#L1`

**Member of:** crustly

### provider

*Rust Module* — `src/llm/provider/mod.rs#L1-L39`

**Imports:** external/pub-use-error-providererror-result, external/pub-use-r-trait-provider-providercapabilities-providerstream, external/pub-use-types, external/pub-use-anthropic-anthropicprovider, external/pub-use-azure-azureopenaiprovider, external/pub-use-factory-create-provider, external/pub-use-factory-ollama-provider-from-config, external/pub-use-gemini-geminiprovider, external/pub-use-ollama-ollamaprovider, external/pub-use-openai-openaiprovider, external/pub-use-qwen-qwenprovider-thinkingconfig-toolcallparser

**Member of:** crustly

### anthropic

*Rust Module* — `src/llm/provider/anthropic.rs#L1-L620`

**Imports:** external/super-error-providererror-result, external/super-r-trait-provider-providerstream, external/super-types, external/async-trait-async-trait, external/futures-stream-streamext, external/reqwest-client, external/serde-deserialize-serialize, external/std-time-duration, external/super-retry-retry-with-backoff-retryconfig, external/super

**Member of:** crustly

### azure

*Rust Module* — `src/llm/provider/azure.rs#L1-L196`

**Imports:** external/super-openai-openaiprovider-llmrequest-llmresponse-provider-result, external/async-trait-async-trait, external/super

**Member of:** crustly

### error

*Rust Module* — `src/llm/provider/error.rs#L1-L128`

**Imports:** external/thiserror-error, external/super

**Member of:** crustly

### factory

*Rust Module* — `src/llm/provider/factory.rs#L1-L1061`

**Imports:** external/super-anthropic-anthropicprovider-azure-azureopenaiprovider-error-providererror-gemini-geminiprovider-openai-openaiprovider-qwen-qwenprovider-toolcallparser-provider, external/crate-config-config-providerconfig-qwenproviderconfig, external/anyhow-context-result, external/async-trait-async-trait, external/std-sync-arc, external/super-ollama-modeloverrides-ollamaprovider, external/super-super-error-providererror-result-as-providerresult-r-trait-providerstream-types-contentblock-llmrequest-llmresponse-stopreason-tokenusage, external/super, external/crate-config-config-providerconfig-providerconfigs-qwenproviderconfig, external/std-sync-atomic-atomicusize-ordering

**Member of:** crustly

### gemini

*Rust Module* — `src/llm/provider/gemini.rs#L1-L1363`

**Imports:** external/super-error-providererror-result, external/super-r-trait-provider-providerstream, external/super-types, external/async-trait-async-trait, external/reqwest-client, external/serde-deserialize-serialize, external/std-collections-hashmap, external/std-time-duration, external/super-retry-retry-with-backoff-retryconfig, external/futures-streamext-as, external/super

**Member of:** crustly

### model_hints

*Rust Module* — `src/llm/provider/model_hints.rs#L1-L47`

**Imports:** external/super

**Member of:** crustly

### ollama

*Rust Module* — `src/llm/provider/ollama.rs#L1-L1804`

**Imports:** external/super-error-providererror-result, external/super-r-trait-provider-providerstream, external/super-types, external/async-trait-async-trait, external/ollama-rs-error-ollamaerror-generation-chat-request-chatmessagerequest-chatmessage-chatmessagefinalresponsedata-chatmessageresponse-messagerole-generation-images-image-generation-parameters-formattype-jsonstructure-keepalive-thinktype-timeunit-generation-tools-toolcall-toolcallfunction-toolfunctioninfo-toolinfo-tooltype-models-modeloptions-ollama, external/futures-streamext-as, external/super

**Member of:** crustly

### ollama_models

*Rust Module* — `src/llm/provider/ollama_models.rs#L1-L349`

**Imports:** external/anyhow-context-result, external/futures-streamext-as, external/ollama-rs-generation-embeddings-request-embeddingsinput-generateembeddingsrequest, external/ollama-rs-ollama, external/tokio-sync-mpsc-unboundedsender, external/super, external/tokio-io-asyncreadext-asyncwriteext

**Member of:** crustly

### openai

*Rust Module* — `src/llm/provider/openai.rs#L1-L1312`

**Imports:** external/super-error-providererror-result, external/super-r-trait-provider-providerstream, external/super-types, external/async-trait-async-trait, external/reqwest-client, external/serde-deserialize-serialize, external/std-time-duration, external/super-retry-retry-with-backoff-retryconfig, external/futures-streamext-as, external/super, external/crate-llm-provider-types-llmrequest-message

**Member of:** crustly

### qwen

*Rust Module* — `src/llm/provider/qwen.rs#L1-L2831`

**Imports:** external/super-error-providererror-result, external/super-r-trait-provider-providerstream, external/super-types, external/async-trait-async-trait, external/futures-stream-streamext, external/reqwest-client, external/serde-deserialize-serialize, external/std-time-duration, external/super-retry-retry-with-backoff-retryconfig, external/super, external/tokio-io-asyncreadext-asyncwriteext

**Member of:** crustly

### retry

*Rust Module* — `src/llm/provider/retry.rs#L1-L421`

**Imports:** external/super-error-providererror-result, external/std-future-future, external/std-time-duration, external/tokio-time-sleep, external/rand-rng, external/regex-regex, external/super, external/std-sync-atomic-atomicu32-ordering, external/std-sync-arc

**Member of:** crustly

### router

*Rust Module* — `src/llm/provider/router.rs#L1-L144`

**Imports:** external/serde-deserialize-serialize, external/super

**Member of:** crustly

### trait

*Rust Module* — `src/llm/provider/trait.rs#L1-L142`

**Imports:** external/super-error-result, external/super-types-llmrequest-llmresponse-streamevent, external/async-trait-async-trait, external/futures-stream, external/std-pin-pin, external/super

**Member of:** crustly

### types

*Rust Module* — `src/llm/provider/types.rs#L1-L611`

**Imports:** external/serde-deserialize-serialize, external/std-collections-hashmap, external/super

**Member of:** crustly

### tools

*Rust Module* — `src/llm/tools/mod.rs#L1-L50`

**Imports:** external/pub-use-error-result-toolerror, external/pub-use-file-read-cache-filefingerprint-filereadcache-readgate, external/pub-use-r-trait-subagentlauncher-tool-toolcapability-toolexecutioncontext-toolresult, external/pub-use-registry-toolregistry

**Member of:** crustly

### agent

*Rust Module* — `src/llm/tools/agent.rs#L1-L258`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/chrono-utc, external/serde-deserialize-serialize, external/serde-json-value, external/uuid-uuid, external/super

**Member of:** crustly

### aliases

*Rust Module* — `src/llm/tools/aliases.rs#L1-L142`

**Imports:** external/super

**Member of:** crustly

### apply_patch

*Rust Module* — `src/llm/tools/apply_patch.rs#L1-L907`

**Imports:** external/super-error-validate-file-path-validate-path-safety-result-toolerror, external/super-file-read-cache-filefingerprint-readgate, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize, external/serde-json-value, external/std-path-pathbuf, external/tokio-fs, external/super, external/tempfile-tempdir, external/uuid-uuid

**Member of:** crustly

### ask_user

*Rust Module* — `src/llm/tools/ask_user.rs#L1-L177`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value, external/std-io-write, external/super

**Member of:** crustly

### bash

*Rust Module* — `src/llm/tools/bash.rs#L1-L735`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value, external/tokio-process-command, external/tokio-time-timeout-duration, external/std-sync-oncelock, external/super, external/uuid-uuid

**Member of:** crustly

### cache

*Rust Module* — `src/llm/tools/cache.rs#L1-L198`

**Imports:** external/dashmap-dashmap, external/serde-json-value, external/std-hash-hash-hasher, external/std-time-duration-instant, external/std-collections-hash-map-defaulthasher, external/super

**Member of:** crustly

### code_exec

*Rust Module* — `src/llm/tools/code_exec.rs#L1-L266`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value, external/tokio-fs, external/tokio-process-command, external/tokio-time-timeout-duration

**Member of:** crustly

### context

*Rust Module* — `src/llm/tools/context.rs#L1-L414`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/chrono-datetime-utc, external/serde-deserialize-serialize, external/serde-json-value, external/std-collections-hashmap, external/std-path-path-pathbuf, external/tokio-fs

**Member of:** crustly

### doc_parser

*Rust Module* — `src/llm/tools/doc_parser.rs#L1-L779`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value, external/std-io-read, external/std-path-path-pathbuf, external/super, external/std-io-write, external/tempfile-tempdir, external/uuid-uuid

**Member of:** crustly

### edit

*Rust Module* — `src/llm/tools/edit.rs#L1-L752`

**Imports:** external/super-error-validate-file-path-result-toolerror, external/super-file-read-cache-filefingerprint-readgate, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value, external/tokio-fs, external/super, external/tempfile-tempdir, external/uuid-uuid

**Member of:** crustly

### error

*Rust Module* — `src/llm/tools/error.rs#L1-L197`

**Imports:** external/thiserror-error, external/std-path-pathbuf, external/super

**Member of:** crustly

### file_read_cache

*Rust Module* — `src/llm/tools/file_read_cache.rs#L1-L141`

**Imports:** external/std-collections-hashmap, external/std-path-path-pathbuf, external/std-sync-mutex, external/std-time-systemtime, external/super

**Member of:** crustly

### glob

*Rust Module* — `src/llm/tools/glob.rs#L1-L294`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value, external/std-path-pathbuf, external/super, external/tempfile-tempdir, external/uuid-uuid

**Member of:** crustly

### grep

*Rust Module* — `src/llm/tools/grep.rs#L1-L486`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value, external/std-path-path-pathbuf, external/tokio-fs, external/super, external/tempfile-tempdir, external/uuid-uuid

**Member of:** crustly

### http

*Rust Module* — `src/llm/tools/http.rs#L1-L362`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/reqwest-header-headermap-client-method, external/serde-deserialize-serialize, external/serde-json-value, external/std-collections-hashmap, external/std-time-duration-as-stdduration, external/super

**Member of:** crustly

### ls

*Rust Module* — `src/llm/tools/ls.rs#L1-L259`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value, external/std-path-path-pathbuf, external/tokio-fs

**Member of:** crustly

### notebook

*Rust Module* — `src/llm/tools/notebook.rs#L1-L409`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value, external/std-path-pathbuf, external/tokio-fs, external/super, external/tempfile-tempdir, external/uuid-uuid

**Member of:** crustly

### plan_tool

*Rust Module* — `src/llm/tools/plan_tool.rs#L1-L1168`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/crate-plan-plandocument-planstatus-plantask-tasktype-toolcall-as-plantoolcall, external/async-trait-async-trait, external/chrono-utc, external/serde-deserialize-serialize, external/serde-json-value, external/std-path-path, external/super, external/tempfile-tempdir

**Member of:** crustly

### plan_tool_security_tests

*Rust Module* — `src/llm/tools/plan_tool_security_tests.rs#L1-L223`

**Imports:** external/super-super, external/std-path-pathbuf, external/tempfile-tempdir, external/std-os-unix-fs-symlink

**Member of:** crustly

### powershell

*Rust Module* — `src/llm/tools/powershell.rs#L1-L524`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/once-cell-sync-lazy, external/serde-deserialize-serialize, external/serde-json-value, external/std-collections-hashmap, external/tokio-process-command, external/tokio-time-timeout-duration, external/super, external/uuid-uuid

**Member of:** crustly

### read

*Rust Module* — `src/llm/tools/read.rs#L1-L402`

**Imports:** external/super-error-validate-file-path-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value, external/tokio-fs, external/tokio-io-asyncbufreadext-bufreader, external/super, external/std-io-write, external/tempfile-tempdir, external/uuid-uuid, external/futures-future-join-all, external/std-sync-arc

**Member of:** crustly

### registry

*Rust Module* — `src/llm/tools/registry.rs#L1-L626`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolexecutioncontext-toolresult, external/serde-json-value, external/std-collections-hashmap, external/std-sync-arc, external/crate-llm-tools-sandbox-policydecision, external/crate-mcp-client-mcpclient-mcptool, external/tokio-sync-mutex, external/super, external/crate-llm-tools-r-trait-toolcapability, external/async-trait-async-trait, external/uuid-uuid, external/crate-llm-tools-sandbox-denytoolrule, external/crate-llm-tools-sandbox-allowtoolrule

**Member of:** crustly

### sandbox

*Rust Module* — `src/llm/tools/sandbox.rs#L1-L885`

**Imports:** external/serde-json-value, external/std-path-path-pathbuf, external/std-path-component-prefix, external/super, external/tempfile-tempdir, external/std-sync-atomic-atomicbool-ordering, external/std-sync-arc

**Member of:** crustly

### save_memory

*Rust Module* — `src/llm/tools/save_memory.rs#L1-L300`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize, external/serde-json-value, external/std-path-path-pathbuf, external/tokio-fs, external/super, external/tempfile-tempdir, external/uuid-uuid

**Member of:** crustly

### skill

*Rust Module* — `src/llm/tools/skill.rs#L1-L459`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value, external/std-path-path-pathbuf, external/super

**Member of:** crustly

### ssrf_guard

*Rust Module* — `src/llm/tools/ssrf_guard.rs#L1-L291`

**Imports:** external/reqwest-dns-addrs-name-resolve-resolving, external/std-net-ipaddr-ipv4addr-ipv6addr-socketaddr, external/std-sync-arc, external/super, external/tokio-io-asyncreadext-asyncwriteext

**Member of:** crustly

### task

*Rust Module* — `src/llm/tools/task.rs#L1-L714`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/chrono-datetime-utc, external/serde-deserialize-serialize, external/serde-json-value, external/std-collections-hashmap, external/std-path-path-pathbuf, external/std-time-duration, external/tokio-fs, external/uuid-uuid, external/tokio-io-asyncwriteext

**Member of:** crustly

### todo_write

*Rust Module* — `src/llm/tools/todo_write.rs#L1-L329`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/chrono-datetime-utc, external/serde-deserialize-serialize, external/serde-json-value, external/std-path-path, external/tokio-fs, external/super

**Member of:** crustly

### trait

*Rust Module* — `src/llm/tools/trait.rs#L1-L254`

**Imports:** external/super-error-result, external/super-file-read-cache-filereadcache, external/async-trait-async-trait, external/serde-json-value, external/std-collections-hashmap, external/std-sync-arc, external/uuid-uuid, external/super

**Member of:** crustly

### web_fetch

*Rust Module* — `src/llm/tools/web_fetch.rs#L1-L312`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/once-cell-sync-lazy, external/regex-regex, external/serde-deserialize-serialize, external/serde-json-value, external/super

**Member of:** crustly

### web_search

*Rust Module* — `src/llm/tools/web_search.rs#L1-L226`

**Imports:** external/super-error-result-toolerror, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value

**Member of:** crustly

### write

*Rust Module* — `src/llm/tools/write.rs#L1-L478`

**Imports:** external/super-error-validate-path-safety-result-toolerror, external/super-file-read-cache-filefingerprint-readgate, external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/serde-deserialize-serialize, external/serde-json-value, external/std-path-pathbuf, external/tokio-fs, external/super, external/tempfile-tempdir, external/uuid-uuid

**Member of:** crustly

### logging

*Rust Module* — `src/logging.rs#L1-L393`

**Imports:** external/std-path-pathbuf, external/tracing-level, external/tracing-appender-non-blocking-workerguard, external/tracing-subscriber-layer-subscriberext-util-subscriberinitext-envfilter, external/super, external/std-io-write

**Member of:** crustly

### lsp

*Rust Module* — `src/lsp/mod.rs#L1-L15`

**Member of:** crustly

### macros

*Rust Module* — `src/macros/mod.rs#L1`

**Member of:** crustly

### mcp

*Rust Module* — `src/mcp/mod.rs#L1-L27`

**Member of:** crustly

### client

*Rust Module* — `src/mcp/client.rs#L1-L602`

**Imports:** external/anyhow-context-result, external/serde-deserialize-serialize, external/serde-json-value, external/tokio-process-command, external/tokio-io-asyncwriteext, external/tokio-io-asyncreadext, external/crate-llm-tools-tool-toolcapability-toolexecutioncontext-toolresult, external/async-trait-async-trait, external/std-sync-arc, external/tokio-sync-mutex, external/super

**Member of:** crustly

### transport

*Rust Module* — `src/mcp/transport/mod.rs#L1-L8`

**Member of:** crustly

### message

*Rust Module* — `src/message/mod.rs#L1-L15`

**Member of:** crustly

### plan

*Rust Module* — `src/plan/mod.rs#L1-L960`

**Imports:** external/chrono-datetime-utc, external/serde-deserialize-serialize, external/uuid-uuid, external/std-collections-hashmap-vecdeque

**Member of:** crustly

### plan_tests

*Rust Module* — `src/plan/plan_tests.rs#L1-L568`

**Imports:** external/crate-plan, external/uuid-uuid

**Member of:** crustly

### services

*Rust Module* — `src/services/mod.rs#L1-L120`

**Imports:** external/pub-use-file-fileservice, external/pub-use-message-messageservice, external/pub-use-plan-planservice, external/pub-use-session-sessionservice, external/crate-db-pool, external/std-sync-arc, external/super, external/crate-db-pool-poolext, external/crate-db-database

**Member of:** crustly

### file

*Rust Module* — `src/services/file.rs#L1-L455`

**Imports:** external/crate-db-models-file-repository-filerepository, external/crate-services-servicecontext, external/anyhow-context-result, external/chrono-utc, external/std-path-path-pathbuf, external/uuid-uuid, external/super, external/crate-services-sessionservice, external/crate-db-database

**Member of:** crustly

### message

*Rust Module* — `src/services/message.rs#L1-L581`

**Imports:** external/crate-db-models-message-repository-messagerepository, external/crate-services-servicecontext, external/anyhow-context-result, external/chrono-utc, external/uuid-uuid, external/super, external/crate-services-sessionservice, external/crate-db-database, external/crate-llm-provider-perfmetrics

**Member of:** crustly

### plan

*Rust Module* — `src/services/plan.rs#L1-L658`

**Imports:** external/crate-db-models-plantaskstatus, external/crate-db-repository-planrepository-plantaskrepository, external/crate-plan-plandocument-planstatus-taskstatus, external/crate-services-servicecontext, external/anyhow-result, external/uuid-uuid, external/super, external/crate-db-models-session, external/crate-db-repository-session-sessionrepository, external/crate-db-database, external/crate-plan-planstatus-plantask-taskstatus-tasktype, external/tempfile-tempdir

**Member of:** crustly

### session

*Rust Module* — `src/services/session.rs#L1-L440`

**Imports:** external/crate-db-models-session-repository-episodicmemoryrepository-sessionlistoptions-sessionrepository, external/crate-llm-agent-memory-episodicmemory, external/crate-llm-provider-types-message, external/crate-services-servicecontext, external/anyhow-context-result, external/chrono-utc, external/uuid-uuid, external/crate-llm-agent-context-token-count, external/crate-llm-provider-types-contentblock, external/crate-llm-provider-types-role, external/super, external/crate-db-database

**Member of:** crustly

### sync

*Rust Module* — `src/sync/mod.rs#L1-L15`

**Member of:** crustly

### tui

*Rust Module* — `src/tui/mod.rs#L1-L28`

**Imports:** external/pub-use-app-app-displaymessage, external/pub-use-events-appmode-eventhandler-tuievent, external/pub-use-prompt-analyzer-promptanalyzer, external/pub-use-runner-run

**Member of:** crustly

### app

*Rust Module* — `src/tui/app.rs#L1-L4099`

**Imports:** external/super-events-appmode-eventhandler-toolapprovalrequest-toolapprovalresponse-tuievent, external/super-prompt-analyzer-promptanalyzer, external/crate-config-planexecmode, external/crate-db-models-message-session, external/crate-llm-agent-agentservice, external/crate-plan-plandocument, external/crate-services-messageservice-planservice-servicecontext-sessionservice, external/anyhow-result, external/ratatui-textarea-cursormove-textarea, external/std-sync-arc-mutex, external/uuid-uuid, external/super-events-keys, external/crossterm-event-keycode-keymodifiers, external/crate-db-repository-sessionlistoptions, external/crossterm-event-keycode, external/super, external/crate-db-database, external/crate-llm-provider-llmrequest-llmresponse-provider-providerstream-result-as-providerresult, external/crossterm-event-keyevent-keyeventkind, external/ratatui-style-style

**Member of:** crustly

### components

*Rust Module* — `src/tui/components/mod.rs#L1-L5`

**Imports:** external/pub-use-logo-get-croissant-get-logo-get-logo-with-version-get-small-logo

**Member of:** crustly

### chat

*Rust Module* — `src/tui/components/chat/mod.rs#L1`

**Member of:** crustly

### dialogs

*Rust Module* — `src/tui/components/dialogs/mod.rs#L1-L171`

**Imports:** external/crate-plan-autorunmode-planmodestate-plantask-taskstatus, external/ratatui-layout-alignment-constraint-direction-layout-rect-style-color-modifier-style-text-line-span-widgets-block-borders-clear-paragraph-wrap-frame

**Member of:** crustly

### logo

*Rust Module* — `src/tui/components/logo.rs#L1-L84`

**Imports:** external/super

**Member of:** crustly

### error

*Rust Module* — `src/tui/error.rs#L1-L297`

**Imports:** external/chrono-datetime-utc, external/ratatui-style-color, external/super

**Member of:** crustly

### events

*Rust Module* — `src/tui/events.rs#L1-L548`

**Imports:** external/crate-llm-agent-agentresponse, external/crossterm-event-keycode-keyevent-keymodifiers, external/serde-json-value, external/std-time-duration, external/tokio-sync-mpsc, external/uuid-uuid, external/super

**Member of:** crustly

### highlight

*Rust Module* — `src/tui/highlight.rs#L1-L214`

**Imports:** external/once-cell-sync-lazy, external/ratatui-style-color-style-text-line-span, external/syntect-easy-highlightlines-highlighting-fontstyle-theme-themeset-parsing-syntaxreference-syntaxset-util-lineswithendings, external/super

**Member of:** crustly

### markdown

*Rust Module* — `src/tui/markdown.rs#L1-L401`

**Imports:** external/pulldown-cmark-codeblockkind-event-parser-tag-tagend, external/ratatui-style-color-modifier-style-text-line-span, external/super-highlight-highlight-code, external/super

**Member of:** crustly

### ollama_download

*Rust Module* — `src/tui/ollama_download.rs#L1-L304`

**Imports:** external/super-events-tuievent, external/tokio-sync-mpsc-unboundedsender, external/tokio-task-joinhandle, external/crate-llm-provider-ollama-models, external/super

**Member of:** crustly

### pages

*Rust Module* — `src/tui/pages/mod.rs#L1`

**Member of:** crustly

### prompt_analyzer

*Rust Module* — `src/tui/prompt_analyzer.rs#L1-L363`

**Imports:** external/regex-regex, external/crate-llm-provider-router-modeltier, external/super

**Member of:** crustly

### render

*Rust Module* — `src/tui/render.rs#L1-L2629`

**Imports:** external/super-app-app, external/super-events-appmode, external/super-markdown-parse-markdown-parse-plain-text, external/super-splash, external/crate-config-planexecmode, external/ratatui-layout-alignment-constraint-direction-layout-rect-style-color-modifier-style-text-line-span-widgets-block-borders-paragraph-wrap-frame, external/super, external/crate-db-database, external/crate-llm-agent-agentservice, external/crate-llm-provider-llmrequest-llmresponse-provider-providerstream-result-as-providerresult, external/crate-services-servicecontext, external/crate-tui-app-displaymessage, external/async-trait-async-trait, external/ratatui-backend-testbackend, external/ratatui-terminal, external/std-sync-arc, external/crate-llm-provider-perfmetrics

**Member of:** crustly

### runner

*Rust Module* — `src/tui/runner.rs#L1-L203`

**Imports:** external/super-app-app, external/super-events-eventhandler, external/super-render, external/anyhow-result, external/crossterm-event-disablebracketedpaste-enablebracketedpaste-keyboardenhancementflags-popkeyboardenhancementflags-pushkeyboardenhancementflags-execute-terminal-disable-raw-mode-enable-raw-mode-supports-keyboard-enhancement-enteralternatescreen-leavealternatescreen, external/ratatui-backend-backend-crosstermbackend-terminal, external/std-io, external/super, external/crate-db-database, external/crate-llm-agent-agentservice, external/crate-llm-provider-llmrequest-llmresponse-provider-providerstream-result-as-providerresult, external/crate-services-servicecontext, external/async-trait-async-trait, external/ratatui-backend-testbackend, external/std-sync-arc

**Member of:** crustly

### splash

*Rust Module* — `src/tui/splash.rs#L1-L170`

**Imports:** external/ratatui-layout-alignment-constraint-direction-layout-rect-style-color-modifier-style-text-line-span-widgets-block-borders-paragraph-frame

**Member of:** crustly

### styles

*Rust Module* — `src/tui/styles/mod.rs#L1`

**Member of:** crustly

### utils

*Rust Module* — `src/tui/utils/mod.rs#L1`

**Member of:** crustly

### utils

*Rust Module* — `src/utils/mod.rs#L1-L45`

**Imports:** external/pub-use-retry-retry-retry-with-check-retryconfig-retryableerror, external/super-truncate-at-char-boundary

**Member of:** crustly

### retry

*Rust Module* — `src/utils/retry.rs#L1-L424`

**Imports:** external/std-future-future, external/std-time-duration, external/tokio-time-sleep, external/rand-rng, external/super, external/std-sync-atomic-atomicu32-ordering, external/std-sync-arc

**Member of:** crustly

### cli_test

*Rust Module* — `tests/cli_test.rs#L1-L273`

**Imports:** external/clap-parser, external/crustly-cli-cli-commands-dbcommands-outputformat

**Member of:** crustly

### codebase_index_test

*Rust Module* — `tests/codebase_index_test.rs#L1-L114`

**Imports:** external/crustly-llm-agent-memory-codebaseindex-symbolkind, external/std-path-path

**Member of:** crustly

### compaction_test

*Rust Module* — `tests/compaction_test.rs#L1-L171`

**Imports:** external/crustly-db-database, external/crustly-llm-agent-compaction-compact, external/crustly-llm-agent-context-agentcontext, external/crustly-llm-provider-types-contentblock-message-role, external/uuid-uuid

**Member of:** crustly

### error_scenarios_test

*Rust Module* — `tests/error_scenarios_test.rs#L1-L362`

**Imports:** external/anyhow-result, external/async-trait-async-trait, external/crustly-db-database-llm-agent-agentservice-provider-error-providererror-result-as-providerresult-types-contentblock-llmrequest-llmresponse-stopreason-tokenusage-provider-providerstream-tools-bash-bashtool-read-readtool-registry-toolregistry-write-writetool-services-servicecontext-sessionservice, external/std-sync-arc, external/uuid-uuid

**Member of:** crustly

### integration_test

*Rust Module* — `tests/integration_test.rs#L1-L463`

**Imports:** external/anyhow-result, external/async-trait-async-trait, external/crustly-config-config-db-database-llm-agent-agentservice-provider-types-contentblock-llmrequest-llmresponse-stopreason-tokenusage-provider-providerstream-tools-bash-bashtool-read-readtool-registry-toolregistry-write-writetool-services-messageservice-servicecontext-sessionservice, external/std-sync-arc, external/uuid-uuid

**Member of:** crustly

### mcp_contract_test

*Rust Module* — `tests/mcp_contract_test.rs#L1-L80`

**Imports:** external/crustly-mcp-client-mcpclient

**Member of:** crustly

### model_routing_test

*Rust Module* — `tests/model_routing_test.rs#L1-L102`

**Imports:** external/crustly-llm-provider-router-modelrouter-modeltier, external/crustly-llm-provider-types-llmrequest, external/crustly-tui-prompt-analyzer-promptanalyzer

**Member of:** crustly

### plan_autorun_test

*Rust Module* — `tests/plan_autorun_test.rs#L1-L141`

**Imports:** external/crustly-plan-autorunmode-planmodestate-plantask-taskstatus-tasktype, external/uuid-uuid

**Member of:** crustly

### plan_crash_recovery_test

*Rust Module* — `tests/plan_crash_recovery_test.rs#L1-L236`

**Imports:** external/crustly-db-models-interrupted-plan-from-tasks-plantaskstatus, external/crustly-db-repository-plantaskrepository, external/crustly-db-database, external/uuid-uuid

**Member of:** crustly

### plan_mode_integration_test

*Rust Module* — `tests/plan_mode_integration_test.rs#L1-L467`

**Imports:** external/crustly-db-models-session, external/crustly-db-repository-session-sessionrepository, external/crustly-db-database, external/crustly-plan-plandocument-planstatus-plantask-taskstatus-tasktype, external/crustly-services-planservice-servicecontext, external/tempfile-tempdir, external/uuid-uuid

**Member of:** crustly

### streaming_test

*Rust Module* — `tests/streaming_test.rs#L1-L341`

**Imports:** external/anyhow-result, external/async-trait-async-trait, external/crustly-llm-provider-error-providererror-result-as-providerresult-types-contentblock-contentdelta-llmrequest-llmresponse-messagedelta-role-stopreason-streamevent-streammessage-tokenusage-provider-providerstream, external/futures-stream-streamext

**Member of:** crustly

## Packages

### crustly

*Rust Package* — `Cargo.toml#L1`

