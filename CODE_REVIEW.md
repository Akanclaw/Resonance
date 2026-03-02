# Resonance 代码审查报告

**审查日期**: 2026-03-02  
**审查范围**: 前端 `src/` 和后端 `src-tauri/src/`  
**代码规模**: ~6,880 行

---

## 🔴 严重问题 (Critical)

### 1. Rust 编译错误 - 重复的 `pause` 方法

**位置**: `src-tauri/src/audio/engine.rs` 第 58-62 行和第 66-70 行

```rust
// 第一次定义
pub fn pause(&mut self) {
    if self.is_playing {
        self.is_paused = true;
    }
}

// 重复定义 (会导致编译错误)
pub fn pause(&mut self) {
    self.is_playing = false;
    info!("Audio paused");
}
```

**影响**: 代码无法编译  
**建议**: 删除重复的方法，保留第一个版本（带状态检查的版本）

---

## 🟠 潜在 Bug

### 2. 批量删除音符时索引失效

**位置**: `src/components/PianoRoll.tsx` 第 375-379 行

```typescript
if (selectedNotes.length > 0 && (e.key === 'Delete' || e.key === 'Backspace')) {
  selectedNotes.forEach(noteIndex => {
    deleteNote(currentTrackIndex, noteIndex);  // ⚠️ 删除后索引会偏移
  });
  clearSelection();
}
```

**问题**: 当删除多个音符时，删除第一个后，后续的索引会失效  
**建议**: 按降序排序后再删除，或先收集要删除的音符再统一处理

### 3. Zustand Store 选中音符索引问题

**位置**: `src/store/projectStore.ts`

同样的问题存在于 `selectedNotes` 的处理中。删除音符后，未重新计算选中状态。

### 4. `getNoteAtPosition` 返回类型不完整

**位置**: `src/components/PianoRoll.tsx` 第 109-127 行

```typescript
const getNoteAtPosition = useCallback((x: number, y: number): { noteIndex: number; type: 'move' | 'resize-left' | 'resize-right' | 'velocity' } | null => {
```

返回类型包含 `'velocity'`，但在代码中从未返回这个类型。

---

## 🟡 代码风格问题

### 5. 魔法数字 (Magic Numbers)

项目中存在大量未命名的常量：

- `480` - 音符默认时长
- `120` - 网格吸附值  
- `36`, `84` - 音高范围
- `120` - BPM 默认值

**建议**: 提取到常量文件或配置中

### 6. 不一致的错误处理

```typescript
// 方式1: 静默忽略
invoke('create_note', {...}).catch(() => {});

// 方式2: 输出到控制台
.catch((err) => console.log('Backend not ready or error:', err));

// 方式3: 完整错误处理
.catch((e) => { console.error('Playback error:', e); });
```

**建议**: 统一错误处理策略

### 7. 缺少 TypeScript 严格模式

部分文件存在 `any` 类型或类型推断问题

---

## 🔵 安全问题

### 8. 缺少输入验证

**位置**: `src-tauri/src/format/io.rs`, `format/midi_io.rs`

从文件读取的数据未经充分验证：
- MIDI 文件解析后无边界检查
- 文件路径未做清理

**建议**: 添加完整的输入验证

### 9. 文件操作路径处理

**位置**: `src/components/Toolbar.tsx`

```typescript
const handleImportMidi = async () => {
  const selected = await open({...});
  // 直接使用用户选择的路径
  const proj = await invoke<Project>('import_midi', { path: selected });
```

**建议**: 验证路径不包含恶意字符

---

## 🟢 改进建议

### 10. 添加 React Error Boundary

当前组件没有错误边界，某个组件崩溃会影响整个应用。

### 11. 加载状态不完整

许多异步操作（如导入/导出文件）缺少加载指示器。

### 12. 历史记录边界情况

**位置**: `src/store/projectStore.ts`

```typescript
const MAX_HISTORY = 50;
```

虽然限制了历史记录大小，但项目数据本身可能很大，50 份完整项目副本会占用显著内存。

### 13. 性能优化 - Piano Roll 渲染

**位置**: `src/components/PianoRoll.tsx`

每次状态变化都触发完整重绘：
```typescript
useEffect(() => {
  render();
}, [render]);
```

**建议**: 考虑使用 `requestAnimationFrame` 或分离渲染层

### 14. 辅助功能 (Accessibility)

按钮缺少 ARIA 标签，键盘导航支持不完整。

### 15. 测试覆盖

- Rust 端有基础测试
- 前端没有测试文件

---

## ✅ 值得肯定的地方

### 1. 项目结构清晰
- 前后端分离良好
- 组件职责分明
- 模块化设计合理

### 2. Rust 代码质量高
- 良好的文档注释
- 完整的 `impl` 方法
- 适当的错误处理 (`thiserror`)
- 单元测试覆盖基础功能

### 3. 状态管理 (Zustand)
- Undo/Redo 实现优雅
- 状态更新不可变
- 类型安全

### 4. 序列化支持
- 完整的 `Serialize`/`Deserialize`
- 与 OpenUtau 格式兼容

### 5. 现代化的开发体验
- 使用 Tauri 2.0
- Vite + React + TypeScript
- Tailwind CSS

---

## 📋 总结

| 类别 | 数量 |
|------|------|
| 🔴 严重问题 | 1 |
| 🟠 潜在 Bug | 4 |
| 🟡 代码风格 | 3 |
| 🔵 安全问题 | 2 |
| 🟢 改进建议 | 5 |

**优先级修复顺序**:
1. 修复 Rust 编译错误（阻塞问题）
2. 修复音符批量删除 bug
3. 统一错误处理
4. 添加输入验证
5. 优化性能和加载状态
