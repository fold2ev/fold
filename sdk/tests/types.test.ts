import { TreeSize } from '../src/types';
import { serialize } from '../src/serialization';

describe('Fold SDK Core', () => {
    describe('Type Definitions', () => {
        it('should have correct depth for MEGA tree', () => {
            expect(TreeSize.MEGA).toBe(24);
        });

        it('should have correct depth for MICRO tree', () => {
            expect(TreeSize.MICRO).toBe(14);
        });
        
        it('should have correct depth for TERA tree', () => {
             expect(TreeSize.TERA).toBe(30);
        });
    });

    describe('Serialization', () => {
        it('should serialize simple objects deterministically', () => {
            const data = { id: 1, name: 'test' };
            const buffer = serialize(data);
            expect(Buffer.isBuffer(buffer)).toBe(true);
            expect(buffer.length).toBeGreaterThan(0);
        });
    });
});
