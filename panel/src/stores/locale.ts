import { ref } from 'vue'
import { defineStore } from 'pinia'
import i18n from '../i18n'

export const useLocaleStore = defineStore('locale', () => {
  const currentLocale = ref(localStorage.getItem('locale') || 'en')

  // Set initial locale for i18n
  if (i18n.global.locale.value !== currentLocale.value) {
    // @ts-ignore
    i18n.global.locale.value = currentLocale.value
  }

  function setLocale(newLocale: string) {
    if (newLocale === 'en' || newLocale === 'zh') {
      currentLocale.value = newLocale
      // @ts-ignore
      i18n.global.locale.value = newLocale
      localStorage.setItem('locale', newLocale)
    }
  }

  return {
    currentLocale,
    setLocale,
  }
})
