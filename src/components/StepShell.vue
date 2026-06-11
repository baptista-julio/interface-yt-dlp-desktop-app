<script setup lang="ts">
// Layout base das "telas simples" (secao 14 do design system):
// centralizado verticalmente, titulo + subtitulo + conteudo, com as
// animacoes de entrada (fade + slide).
//
// Slots:
//   default  -> conteudo interativo (form, botoes...)
//   actions  -> rodape com botoes de navegacao (opcional)

defineProps<{
  title: string;
  subtitle?: string;
  /** Indica o passo atual, ex: "Passo 1 de 5" (opcional). */
  stepLabel?: string;
}>();
</script>

<template>
  <div class="step-shell">
    <div class="step-body">
      <div class="step-head">
        <span v-if="stepLabel" class="step-label">{{ stepLabel }}</span>
        <h1>{{ title }}</h1>
        <p v-if="subtitle" class="step-subtitle">{{ subtitle }}</p>
      </div>

      <div class="step-content">
        <slot />
      </div>
    </div>

    <div v-if="$slots.actions" class="step-actions">
      <slot name="actions" />
    </div>
  </div>
</template>

<style scoped>
.step-shell {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
}

.step-body {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1.75rem;
  text-align: center;
  overflow-y: auto;
  padding: 0.5rem 0;
}

.step-head {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  animation: fadeSlideDown 0.4s ease-out both;
}

.step-label {
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--primary);
}

.step-subtitle {
  font-size: 1rem;
  color: var(--text-secondary);
  max-width: 28rem;
}

.step-content {
  width: 100%;
  max-width: 420px;
  display: flex;
  flex-direction: column;
  align-items: stretch;
  gap: 1rem;
  animation: fadeSlideUp 0.4s ease-out 0.1s both;
}

.step-actions {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  padding-top: 1.25rem;
  animation: fadeSlideUp 0.4s ease-out 0.15s both;
}

@keyframes fadeSlideDown {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes fadeSlideUp {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@media (max-width: 600px) {
  .step-body {
    gap: 1.25rem;
  }
}
</style>
