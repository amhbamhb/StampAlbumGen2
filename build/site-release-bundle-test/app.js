const cards = document.querySelectorAll('.panel, .stamp-card');

const observer = new IntersectionObserver((entries) => {
  entries.forEach((entry) => {
    if (entry.isIntersecting) {
      entry.target.classList.add('in');
      observer.unobserve(entry.target);
    }
  });
}, { threshold: 0.18 });

cards.forEach((card, idx) => {
  card.classList.add('reveal');
  card.style.transitionDelay = `${Math.min(idx * 70, 500)}ms`;
  observer.observe(card);
});

const currentYear = String(new Date().getFullYear());
document.querySelectorAll('.js-current-year').forEach((node) => {
  node.textContent = currentYear;
});

wireCopyButtons();

loadAppVersion();

const isLocalDevHost = ['localhost', '127.0.0.1', '::1'].includes(window.location.hostname);

if (isLocalDevHost) {
  enableRestartAutoRefresh();
}

function enableRestartAutoRefresh() {
  let sawServerDown = false;

  window.setInterval(async () => {
    try {
      const response = await fetch('/__heartbeat', { cache: 'no-store' });
      if (!response.ok) {
        throw new Error('heartbeat not ok');
      }

      if (sawServerDown) {
        window.location.reload();
      }

      sawServerDown = false;
    } catch {
      sawServerDown = true;
    }
  }, 1100);
}

async function loadAppVersion() {
  try {
    const response = await fetch('/api/version', { cache: 'no-store' });
    if (!response.ok) {
      return;
    }

    const version = (await response.text()).trim();
    if (!version) {
      return;
    }

    document.querySelectorAll('.js-app-version').forEach((node) => {
      node.textContent = version;
    });
  } catch {
    // Keep fallback version text in HTML when API is temporarily unavailable.
  }
}

function wireCopyButtons() {
  const copyButtons = document.querySelectorAll('.copy-btn[data-copy-target]');
  copyButtons.forEach((button) => {
    button.addEventListener('click', async () => {
      const targetId = button.getAttribute('data-copy-target');
      const source = targetId ? document.getElementById(targetId) : null;
      if (!source) {
        return;
      }

      const text = source.textContent ? source.textContent.trim() : '';
      if (!text) {
        return;
      }

      const copied = await copyText(text);
      if (!copied) {
        return;
      }

      const previousLabel = button.textContent;
      button.textContent = 'Copied';
      button.classList.add('copied');

      window.setTimeout(() => {
        button.textContent = previousLabel;
        button.classList.remove('copied');
      }, 1200);
    });
  });
}

async function copyText(text) {
  try {
    if (navigator.clipboard && window.isSecureContext) {
      await navigator.clipboard.writeText(text);
      return true;
    }
  } catch {
    // Fall through to legacy copy path.
  }

  const area = document.createElement('textarea');
  area.value = text;
  area.setAttribute('readonly', '');
  area.style.position = 'fixed';
  area.style.left = '-9999px';
  document.body.appendChild(area);
  area.select();

  let ok = false;
  try {
    ok = document.execCommand('copy');
  } catch {
    ok = false;
  }

  document.body.removeChild(area);
  return ok;
}

