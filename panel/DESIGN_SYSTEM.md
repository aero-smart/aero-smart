# AeroSmart Design System

## 1. Introduction

This document outlines the design system for the AeroSmart control panel. The system is built on Tailwind CSS and provides a consistent set of tokens, components, and layout rules to ensure a unified user experience.

## 2. Design Tokens

### Colors

The color palette is semantic and supports theming via CSS variables.

| Token       | Variable            | Description                        |
| ----------- | ------------------- | ---------------------------------- |
| `primary`   | `--color-primary`   | Main action color (Black)          |
| `secondary` | `--color-secondary` | Secondary backgrounds (Light Gray) |
| `accent`    | `--color-accent`    | Highlight color (Blue)             |
| `success`   | `--color-success`   | Success state (Green)              |
| `warning`   | `--color-warning`   | Warning state (Yellow)             |
| `danger`    | `--color-danger`    | Error/Destructive state (Red)      |

### Backgrounds

| Token        | Variable       | Description                 |
| ------------ | -------------- | --------------------------- |
| `bg-page`    | `--bg-page`    | Main application background |
| `bg-surface` | `--bg-surface` | Card/Panel background       |
| `bg-sidebar` | `--bg-sidebar` | Sidebar background          |

### Typography

We use the system font stack with Inter as the preferred font.

- **Font Family**: Inter, -apple-system, BlinkMacSystemFont, ...
- **Weights**: Regular (400), Medium (500), Semibold (600), Bold (700)

### Spacing

Based on an 4px grid system.

- `p-1` = 4px
- `p-2` = 8px
- `p-4` = 16px
- `p-6` = 24px

## 3. Components

### Button (`@/components/ui/Button.vue`)

Standard button component with variants.

```vue
<Button variant="primary">Primary Action</Button>
<Button variant="outline">Secondary Action</Button>
<Button variant="ghost" size="icon"><Icon /></Button>
```

### Card (`@/components/ui/Card.vue`)

Content container with consistent styling.

```vue
<Card title="Card Title">
  <p>Content goes here...</p>
</Card>
```

### Badge (`@/components/ui/Badge.vue`)

Status indicator.

```vue
<Badge variant="success" dot>Online</Badge>
```

### Input (`@/components/ui/Input.vue`)

Standard text input.

```vue
<Input v-model="value" label="Username" placeholder="Enter username" />
```

## 4. Layout

### PageLayout (`@/components/layout/PageLayout.vue`)

Standard page structure with header and scrollable content area.

```vue
<PageLayout title="Dashboard">
  <template #actions>
    <Button>Action</Button>
  </template>
  <!-- Content -->
</PageLayout>
```

### Container (`@/components/layout/Container.vue`)

Centers content with max-width and padding.

### Grid (`@/components/layout/Grid.vue`)

Responsive grid layout wrapper.

```vue
<Grid :cols="3">
  <Card>...</Card>
  <Card>...</Card>
  <Card>...</Card>
</Grid>
```

## 5. Iconography

We use **Lucide Vue Next** for icons.

- **Style**: Line style, 2px stroke width.
- **Size**: Default to `16px` (sm), `20px` (md), `24px` (lg).
- **Color**: Use `text-gray-400` for inactive/neutral icons, `text-primary` for active.

## 6. Guidelines

1.  **Use Semantic Colors**: Avoid hardcoded hex values (e.g., use `text-primary` instead of `#1f2937`).
2.  **Use Spacing Scale**: Use Tailwind classes (`p-4`, `m-2`) instead of arbitrary pixels.
3.  **Consistent Cards**: Wrap distinct content sections in `Card` components.
4.  **Responsive First**: Build mobile-first layouts using standard breakpoints (`md`, `lg`).

## 6. Accessibility

- Ensure sufficient color contrast for text.
- Use semantic HTML elements.
- Provide aria-labels for icon-only buttons.
