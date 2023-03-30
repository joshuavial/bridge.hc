import { LitElement, html, css } from 'lit';
import { customElement } from 'lit/decorators.js';

@customElement('header-component')
class HeaderComponent extends LitElement {
  render() {
    return html`
      <h1>HoloScan</h1>
    `;
  }

  static styles = css`
    :host {
        text-align: left;
        color: #1d6fa5;
        border: 2px solid #1d6fa5;
        border-top: 0;
        border-left: 0;
        flex-grow: 0;
        display: flex;
    }

    h1 {
      margin: 0;
    }

  `;
}