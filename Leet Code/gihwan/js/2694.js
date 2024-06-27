class EventEmitter {
  #eventStore = new Map();

  /**
   * @param {string} eventName
   * @param {Function} callback
   * @return {Object}
   */
  subscribe(eventName, callback) {
    const prev = this.#eventStore.get(eventName);

    if (!prev) {
      this.#eventStore.set(eventName, [callback]);
    } else {
      this.#eventStore.set(eventName, [...prev, callback]);
    }

    return {
      unsubscribe: () => {
        const cur = this.#eventStore.get(eventName);
        const filtered = cur.filter((cb) => cb !== callback);
        this.#eventStore.set(eventName, filtered);
      },
    };
  }

  /**
   * @param {string} eventName
   * @param {Array} args
   * @return {Array}
   */
  emit(eventName, args = []) {
    return (
      this.#eventStore.get(eventName)?.map((callback) => callback(...args)) ??
      []
    );
  }
}

/**
 * const emitter = new EventEmitter();
 *
 * // Subscribe to the onClick event with onClickCallback
 * function onClickCallback() { return 99 }
 * const sub = emitter.subscribe('onClick', onClickCallback);
 *
 * emitter.emit('onClick'); // [99]
 * sub.unsubscribe(); // undefined
 * emitter.emit('onClick'); // []
 */
