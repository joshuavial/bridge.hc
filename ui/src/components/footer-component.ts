import { LitElement, html, css } from 'lit';
import { customElement } from 'lit/decorators.js';

@customElement('footer-component')
class FooterComponent extends LitElement {
  render() {
    return html`
      <span>Authorities List</span>
    `;
  }

  static styles = css`
    :host {
      background-color: #a3f3f3
    }
`;
}
